// Generated macro for enc_arith_rr_imm12 (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_arith_rr_imm12 {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_arith_rr_imm12"}
// Dependencies: {}
fn enc_arith_rr_imm12 (bits_31_24 : u32 , immshift : u32 , imm12 : u32 , rn : Reg , rd : Writable < Reg > ,) -> u32 { (bits_31_24 << 24) | (immshift << 22) | (imm12 << 10) | (machreg_to_gpr (rn) << 5) | machreg_to_gpr (rd . to_reg ()) }
};
}
