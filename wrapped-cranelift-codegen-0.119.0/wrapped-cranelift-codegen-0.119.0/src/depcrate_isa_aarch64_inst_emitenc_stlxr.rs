// Generated macro for enc_stlxr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_stlxr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_stlxr"}
// Dependencies: {}
fn enc_stlxr (ty : Type , rs : Writable < Reg > , rt : Reg , rn : Reg) -> u32 { let sz = match ty { I64 => 0b11 , I32 => 0b10 , I16 => 0b01 , I8 => 0b00 , _ => unreachable ! () , } ; 0b00_001000_000_00000_1_11111_00000_00000 | (sz << 30) | (machreg_to_gpr (rs . to_reg ()) << 16) | (machreg_to_gpr (rn) << 5) | machreg_to_gpr (rt) }
};
}
