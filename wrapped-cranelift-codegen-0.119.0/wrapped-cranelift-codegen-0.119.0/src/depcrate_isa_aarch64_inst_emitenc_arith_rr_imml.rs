// Generated macro for enc_arith_rr_imml (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_arith_rr_imml {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_arith_rr_imml"}
// Dependencies: {}
fn enc_arith_rr_imml (bits_31_23 : u32 , imm_bits : u32 , rn : Reg , rd : Writable < Reg >) -> u32 { (bits_31_23 << 23) | (imm_bits << 10) | (machreg_to_gpr (rn) << 5) | machreg_to_gpr (rd . to_reg ()) }
};
}
