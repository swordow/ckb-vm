use super::{
    super::{machine::Machine, Error},
    common, extract_opcode, instruction_length,
    utils::update_register,
    Instruction, Itype, R4type, Register, Rtype, Stype, Utype,
};
use ckb_vm_definitions::{instructions as insts, registers::RA};

pub fn execute_instruction<Mac: Machine>(
    inst: Instruction,
    machine: &mut Mac,
) -> Result<(), Error> {
    let op = extract_opcode(inst);
    match op {
        insts::OP_SUB => {
            let i = Rtype(inst);
            common::sub(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_SUBW => {
            let i = Rtype(inst);
            common::subw(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_ADD => {
            let i = Rtype(inst);
            common::add(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_ADDW => {
            let i = Rtype(inst);
            common::addw(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_XOR => {
            let i = Rtype(inst);
            common::xor(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_OR => {
            let i = Rtype(inst);
            common::or(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_AND => {
            let i = Rtype(inst);
            common::and(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_SLL => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].clone() << shift_value;
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLLW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
            let value = machine.registers()[i.rs1()].clone() << shift_value;
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SRL => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].clone() >> shift_value;
            update_register(machine, i.rd(), value);
        }
        insts::OP_SRLW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
            let value =
                machine.registers()[i.rs1()].zero_extend(&Mac::REG::from_u8(32)) >> shift_value;
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SRA => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].signed_shr(&shift_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SRAW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
            let value = machine.registers()[i.rs1()]
                .sign_extend(&Mac::REG::from_u8(32))
                .signed_shr(&shift_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SLT => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt_s(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLTU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_LB => {
            let i = Itype(inst);
            common::lb(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LH => {
            let i = Itype(inst);
            common::lh(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LW => {
            let i = Itype(inst);
            common::lw(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LD => {
            let i = Itype(inst);
            common::ld(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LBU => {
            let i = Itype(inst);
            common::lbu(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LHU => {
            let i = Itype(inst);
            common::lhu(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LWU => {
            let i = Itype(inst);
            common::lwu(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_ADDI => {
            let i = Itype(inst);
            common::addi(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_ADDIW => {
            let i = Itype(inst);
            common::addiw(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_XORI => {
            let i = Itype(inst);
            common::xori(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_ORI => {
            let i = Itype(inst);
            common::ori(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_ANDI => {
            let i = Itype(inst);
            common::andi(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_SLTI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt_s(&imm_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLTIU => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt(&imm_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_JALR => {
            let i = Itype(inst);
            let size = instruction_length(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
            if machine.version() >= 1 {
                let mut next_pc = machine.registers()[i.rs1()]
                    .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
                next_pc = next_pc & (!Mac::REG::one());
                update_register(machine, i.rd(), link);
                machine.update_pc(next_pc);
            } else {
                update_register(machine, i.rd(), link);
                let mut next_pc = machine.registers()[i.rs1()]
                    .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
                next_pc = next_pc & (!Mac::REG::one());
                machine.update_pc(next_pc);
            }
        }
        insts::OP_SLLI => {
            let i = Itype(inst);
            common::slli(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRLI => {
            let i = Itype(inst);
            common::srli(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRAI => {
            let i = Itype(inst);
            common::srai(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SLLIW => {
            let i = Itype(inst);
            common::slliw(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRLIW => {
            let i = Itype(inst);
            common::srliw(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRAIW => {
            let i = Itype(inst);
            common::sraiw(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SB => {
            let i = Stype(inst);
            common::sb(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_SH => {
            let i = Stype(inst);
            common::sh(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_SW => {
            let i = Stype(inst);
            common::sw(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_SD => {
            let i = Stype(inst);
            common::sd(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_BEQ => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.eq(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_BNE => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.ne(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_BLT => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.lt_s(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_BGE => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.ge_s(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_BLTU => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.lt(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_BGEU => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.ge(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_LUI => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
        }
        insts::OP_AUIPC => {
            let i = Utype(inst);
            let value = machine
                .pc()
                .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
            update_register(machine, i.rd(), value);
        }
        insts::OP_ECALL => {
            // The semantic of ECALL is determined by the hardware, which
            // is not part of the spec, hence here the implementation is
            // deferred to the machine. This way custom ECALLs might be
            // provided for different environments.
            machine.ecall()?;
        }
        insts::OP_EBREAK => {
            machine.ebreak()?;
        }
        insts::OP_FENCEI => {}
        insts::OP_FENCE => {}
        insts::OP_JAL => {
            let i = Utype(inst);
            common::jal(machine, i.rd(), i.immediate_s(), instruction_length(inst));
        }
        insts::OP_MUL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&rs2_value.zero_extend(&Mac::REG::from_u8(32)));
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_MULH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULHSU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_signed_unsigned(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULHU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_unsigned(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIV => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_div_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIVW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.sign_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.sign_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_div_signed(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_DIVU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_div(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIVUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.zero_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.zero_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_div(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_REM => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_REMW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.sign_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.sign_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_REMU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_rem(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_REMUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.zero_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.zero_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_rem(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_CLZ => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.clz();
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLZW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .clz()
                .overflowing_sub(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_CTZ => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.ctz();
            update_register(machine, i.rd(), value);
        }
        insts::OP_CTZW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = (rs1_value.clone() | Mac::REG::from_u64(0xffff_ffff_0000_0000)).ctz();
            update_register(machine, i.rd(), value);
        }
        insts::OP_PCNT => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.pcnt();
            update_register(machine, i.rd(), value);
        }
        insts::OP_PCNTW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.zero_extend(&Mac::REG::from_u8(32)).pcnt();
            update_register(machine, i.rd(), value);
        }
        insts::OP_ANDN => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clone() & !rs2_value.clone();
            update_register(machine, i.rd(), value);
        }
        insts::OP_ORN => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clone() | !rs2_value.clone();
            update_register(machine, i.rd(), value);
        }
        insts::OP_XNOR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clone() ^ !rs2_value.clone();
            update_register(machine, i.rd(), value);
        }
        insts::OP_PACK => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let xlen_half = Mac::REG::from_u8(Mac::REG::BITS / 2);
            let upper = rs2_value.clone() << xlen_half.clone();
            let lower = rs1_value.clone() << xlen_half.clone() >> xlen_half;
            let value = upper | lower;
            update_register(machine, i.rd(), value);
        }
        insts::OP_PACKU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let xlen_half = Mac::REG::from_u8(Mac::REG::BITS / 2);
            let upper = rs2_value.clone() >> xlen_half.clone() << xlen_half.clone();
            let lower = rs1_value.clone() >> xlen_half;
            let value = upper | lower;
            update_register(machine, i.rd(), value);
        }
        insts::OP_PACKH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let upper = (rs2_value.clone() & Mac::REG::from_u8(0xff)) << Mac::REG::from_u8(8);
            let lower = rs1_value.clone() & Mac::REG::from_u8(0xff);
            let value = upper | lower;
            update_register(machine, i.rd(), value);
        }
        insts::OP_PACKW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let upper = rs2_value.clone() << Mac::REG::from_u8(16);
            let lower = rs1_value.zero_extend(&Mac::REG::from_u8(16));
            let value = (upper | lower).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_PACKUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let upper = rs2_value.sign_extend(&Mac::REG::from_u8(32))
                & Mac::REG::from_u64(0xffff_ffff_ffff_0000);
            let lower = rs1_value.clone() << Mac::REG::from_u8(32) >> Mac::REG::from_u8(48);
            let value = upper | lower;
            update_register(machine, i.rd(), value);
        }
        insts::OP_MIN => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt_s(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MINU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MAX => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.ge_s(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MAXU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.ge(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SEXTB => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let shift = &Mac::REG::from_u8(Mac::REG::BITS - 8);
            let value = rs1_value.signed_shl(shift).signed_shr(shift);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SEXTH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let shift = &Mac::REG::from_u8(Mac::REG::BITS - 16);
            let value = rs1_value.signed_shl(shift).signed_shr(shift);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SBSET => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() | (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SBSETI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() | (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SBSETW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let value = rs1_value.clone() | (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SBSETIW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let value = rs1_value.clone() | (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SBCLR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() & !(Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SBCLRI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() & !(Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SBCLRW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let value = rs1_value.clone() & Mac::REG::from_u64(0xffff_fffe_ffff_fffe).rol(&shamt);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SBCLRIW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let value = rs1_value.clone() & Mac::REG::from_u64(0xffff_fffe_ffff_fffe).rol(&shamt);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SBINV => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() ^ (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SBINVI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() ^ (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SBINVW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let value = rs1_value.clone() ^ (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SBINVIW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let value = rs1_value.clone() ^ (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SBEXT => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = Mac::REG::one() & (rs1_value.clone() >> shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SBEXTI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = Mac::REG::one() & (rs1_value.clone() >> shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SBEXTW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let value = Mac::REG::one() & (rs1_value.clone() >> shamt);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SLO => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.slo(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLOI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.slo(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLOW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let value = rs1_value.slo(&shamt).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLOIW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let value = rs1_value.slo(&shamt).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_SRO => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.sro(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SROI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.sro(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SROW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let value = (rs1_value.clone() | Mac::REG::from_u64(0xffff_ffff_0000_0000)) >> shamt;
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SROIW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let value = (rs1_value.clone() | Mac::REG::from_u64(0xffff_ffff_0000_0000)) >> shamt;
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_ROR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.ror(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_RORI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.ror(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_RORW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let twins = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
            let value = twins.ror(&shamt).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_RORIW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let twins = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
            let value = twins.ror(&shamt).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_ROL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.rol(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_ROLW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let twins = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
            let value = twins.rol(&shamt).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_GREV => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::grev32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::grev64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_GREVI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = i.immediate();
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::grev32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::grev64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_GREVW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = common::grev32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_GREVIW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = i.immediate();
            let value = common::grev32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_SHFL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::shfl32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::shfl64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_UNSHFL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::unshfl32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::unshfl64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_SHFLI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = i.immediate();
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::shfl32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::shfl64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_UNSHFLI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = i.immediate();
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::unshfl32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::unshfl64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_SHFLW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = common::shfl32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_UNSHFLW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = common::unshfl32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_GORC => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::gorc32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::gorc64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_GORCI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = i.immediate();
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::gorc32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::gorc64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_GORCW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = common::gorc32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_GORCIW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = i.immediate();
            let value = common::gorc32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_BFP => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::bfp32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::bfp64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_BFPW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = common::bfp32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_BEXT => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::bext32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::bext64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_BEXTW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = common::bext32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_BDEP => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::bdep32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::bdep64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_BDEPW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = common::bdep32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_CLMUL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::clmul32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::clmul64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLMULW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = common::clmul32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_CLMULH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::clmulh32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::clmulh64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLMULHW => {
            let i = Rtype(inst);
            let rs1_value = machine.registers()[i.rs1()].to_u32();
            let rs2_value = machine.registers()[i.rs2()].to_u32();
            let value = common::clmulh32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_CLMULR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::clmulr32(rs1_value.to_u32(), rs2_value.to_u32()))
            } else {
                Mac::REG::from_u64(common::clmulr64(rs1_value.to_u64(), rs2_value.to_u64()))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLMULRW => {
            let i = Rtype(inst);
            let rs1_value = machine.registers()[i.rs1()].to_u32();
            let rs2_value = machine.registers()[i.rs2()].to_u32();
            let value = common::clmulr32(rs1_value.to_u32(), rs2_value.to_u32());
            let r = Mac::REG::from_u32(value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), r);
        }
        insts::OP_CRC32B => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::crc3232(rs1_value.to_u32(), 8))
            } else {
                Mac::REG::from_u64(common::crc3264(rs1_value.to_u64(), 8))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_CRC32H => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::crc3232(rs1_value.to_u32(), 16))
            } else {
                Mac::REG::from_u64(common::crc3264(rs1_value.to_u64(), 16))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_CRC32W => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::crc3232(rs1_value.to_u32(), 32))
            } else {
                Mac::REG::from_u64(common::crc3264(rs1_value.to_u64(), 32))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_CRC32CB => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::crc32c32(rs1_value.to_u32(), 8))
            } else {
                Mac::REG::from_u64(common::crc32c64(rs1_value.to_u64(), 8))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_CRC32CH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::crc32c32(rs1_value.to_u32(), 16))
            } else {
                Mac::REG::from_u64(common::crc32c64(rs1_value.to_u64(), 16))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_CRC32CW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::crc32c32(rs1_value.to_u32(), 32))
            } else {
                Mac::REG::from_u64(common::crc32c64(rs1_value.to_u64(), 32))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_CRC32D => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::crc3232(rs1_value.to_u32(), 64))
            } else {
                Mac::REG::from_u64(common::crc3264(rs1_value.to_u64(), 64))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_CRC32CD => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = if Mac::REG::BITS == 32 {
                Mac::REG::from_u32(common::crc32c32(rs1_value.to_u32(), 64))
            } else {
                Mac::REG::from_u64(common::crc32c64(rs1_value.to_u64(), 64))
            };
            update_register(machine, i.rd(), value);
        }
        insts::OP_BMATOR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = common::bmator(rs1_value.to_u64(), rs2_value.to_u64());
            update_register(machine, i.rd(), Mac::REG::from_u64(value));
        }
        insts::OP_BMATXOR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = common::bmatxor(rs1_value.to_u64(), rs2_value.to_u64());
            update_register(machine, i.rd(), Mac::REG::from_u64(value));
        }
        insts::OP_BMATFLIP => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = common::bmatflip(rs1_value.to_u64());
            update_register(machine, i.rd(), Mac::REG::from_u64(value));
        }
        insts::OP_CMIX => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs3_value = &machine.registers()[i.rs3()];
            let value =
                (rs1_value.clone() & rs2_value.clone()) | (rs3_value.clone() & !rs2_value.clone());
            update_register(machine, i.rd(), value);
        }
        insts::OP_CMOV => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs3_value = &machine.registers()[i.rs3()];
            let value = rs2_value.eq(&Mac::REG::zero()).cond(rs3_value, rs1_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_FSL => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs3_value = &machine.registers()[i.rs3()];
            let value = rs1_value.fsl(rs3_value, rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_FSLW => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs3_value = &machine.registers()[i.rs3()];
            let upper = rs1_value.clone() << Mac::REG::from_u8(32);
            let lower = rs3_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = upper | lower;
            let value = value.rol(rs2_value).signed_shr(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_FSR => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs3_value = &machine.registers()[i.rs3()];
            let value = rs1_value.fsr(rs3_value, rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_FSRW => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs3_value = &machine.registers()[i.rs3()];
            let upper = rs3_value.clone() << Mac::REG::from_u8(32);
            let lower = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = upper | lower;
            let value = value.ror(rs2_value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_FSRI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let immediate = i.immediate();
            let rs2_value = &Mac::REG::from_u32(immediate & 0x3f);
            let rs3_value = &machine.registers()[immediate as usize >> 7];
            let value = rs1_value.fsr(rs3_value, rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_FSRIW => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.rs2() as u32);
            let rs3_value = &machine.registers()[i.rs3()];
            let upper = rs3_value.clone() << Mac::REG::from_u8(32);
            let lower = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = upper | lower;
            let value = value.ror(rs2_value).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH1ADD => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = (rs1_value.clone() << Mac::REG::from_u32(1)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH2ADD => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = (rs1_value.clone() << Mac::REG::from_u32(2)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH3ADD => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = (rs1_value.clone() << Mac::REG::from_u32(3)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH1ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = (rs1_z << Mac::REG::from_u32(1)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH2ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = (rs1_z << Mac::REG::from_u32(2)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH3ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = (rs1_z << Mac::REG::from_u32(3)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_ADDWU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value
                .overflowing_add(&rs2_value)
                .zero_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_SUBWU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value
                .overflowing_sub(&rs2_value)
                .zero_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_ADDIWU => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value
                .overflowing_add(rs2_value)
                .zero_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs2_u = rs2_value.zero_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_add(&rs2_u);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SUBUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs2_u = rs2_value.zero_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_sub(&rs2_u);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLLIUW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = Mac::REG::from_u32(i.immediate());
            let rs1_u = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let shamt = rs2_value & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_u << shamt;
            update_register(machine, i.rd(), value);
        }
        insts::OP_WIDE_MUL => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_mul_high_signed(&rs2_value);
            let value_l = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_MULU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_mul_high_unsigned(&rs2_value);
            let value_l = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_MULSU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_mul_high_signed_unsigned(&rs2_value);
            let value_l = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_DIV => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_div_signed(&rs2_value);
            let value_l = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_DIVU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_div(&rs2_value);
            let value_l = rs1_value.overflowing_rem(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_FAR_JUMP_REL => {
            let i = Utype(inst);
            let size = instruction_length(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
            let next_pc = machine
                .pc()
                .overflowing_add(&Mac::REG::from_i32(i.immediate_s()))
                & (!Mac::REG::one());
            update_register(machine, RA, link);
            machine.update_pc(next_pc);
        }
        insts::OP_FAR_JUMP_ABS => {
            let i = Utype(inst);
            let size = instruction_length(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
            let next_pc = Mac::REG::from_i32(i.immediate_s()) & (!Mac::REG::one());
            update_register(machine, RA, link);
            machine.update_pc(next_pc);
        }
        insts::OP_LD_SIGN_EXTENDED_32_CONSTANT => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
        }
        insts::OP_CUSTOM_LOAD_IMM => {
            let i = Utype(inst);
            let value = Mac::REG::from_i32(i.immediate_s());
            update_register(machine, i.rd(), value);
        }
        _ => return Err(Error::InvalidOp(op)),
    };
    Ok(())
}

pub fn execute<Mac: Machine>(inst: Instruction, machine: &mut Mac) -> Result<(), Error> {
    let instruction_size = instruction_length(inst);
    let next_pc = machine
        .pc()
        .overflowing_add(&Mac::REG::from_u8(instruction_size));
    machine.update_pc(next_pc);
    execute_instruction(inst, machine)?;
    machine.commit_pc();
    Ok(())
}
