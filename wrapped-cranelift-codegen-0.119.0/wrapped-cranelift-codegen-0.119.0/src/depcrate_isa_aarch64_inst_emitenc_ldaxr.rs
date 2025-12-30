// Generated macro for enc_ldaxr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_ldaxr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_ldaxr"}
// Dependencies: {}
fn enc_ldaxr (ty : Type , rt : Writable < Reg > , rn : Reg) -> u32 { let sz = match ty { I64 => 0b11 , I32 => 0b10 , I16 => 0b01 , I8 => 0b00 , _ => unreachable ! () , } ; 0b00_001000_0_1_0_11111_1_11111_00000_00000 | (sz << 30) | (machreg_to_gpr (rn) << 5) | machreg_to_gpr (rt . to_reg ()) }
};
}
