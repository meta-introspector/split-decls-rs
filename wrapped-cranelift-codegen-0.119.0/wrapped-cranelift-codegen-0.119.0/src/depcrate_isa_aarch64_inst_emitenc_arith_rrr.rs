// Generated macro for enc_arith_rrr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_arith_rrr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_arith_rrr"}
// Dependencies: {}
pub (crate) fn enc_arith_rrr (bits_31_21 : u32 , bits_15_10 : u32 , rd : Writable < Reg > , rn : Reg , rm : Reg ,) -> u32 { (bits_31_21 << 21) | (bits_15_10 << 10) | machreg_to_gpr (rd . to_reg ()) | (machreg_to_gpr (rn) << 5) | (machreg_to_gpr (rm) << 16) }
};
}
