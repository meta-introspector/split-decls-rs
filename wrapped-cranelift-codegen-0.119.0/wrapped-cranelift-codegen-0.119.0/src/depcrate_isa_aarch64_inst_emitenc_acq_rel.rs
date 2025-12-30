// Generated macro for enc_acq_rel (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_acq_rel {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_acq_rel"}
// Dependencies: {}
fn enc_acq_rel (ty : Type , op : AtomicRMWOp , rs : Reg , rt : Writable < Reg > , rn : Reg) -> u32 { assert ! (machreg_to_gpr (rt . to_reg ()) != 31) ; let sz = match ty { I64 => 0b11 , I32 => 0b10 , I16 => 0b01 , I8 => 0b00 , _ => unreachable ! () , } ; let bit15 = match op { AtomicRMWOp :: Swp => 0b1 , _ => 0b0 , } ; let op = match op { AtomicRMWOp :: Add => 0b000 , AtomicRMWOp :: Clr => 0b001 , AtomicRMWOp :: Eor => 0b010 , AtomicRMWOp :: Set => 0b011 , AtomicRMWOp :: Smax => 0b100 , AtomicRMWOp :: Smin => 0b101 , AtomicRMWOp :: Umax => 0b110 , AtomicRMWOp :: Umin => 0b111 , AtomicRMWOp :: Swp => 0b000 , } ; 0b00_111_000_111_00000_0_000_00_00000_00000 | (sz << 30) | (machreg_to_gpr (rs) << 16) | bit15 << 15 | (op << 12) | (machreg_to_gpr (rn) << 5) | machreg_to_gpr (rt . to_reg ()) }
};
}
