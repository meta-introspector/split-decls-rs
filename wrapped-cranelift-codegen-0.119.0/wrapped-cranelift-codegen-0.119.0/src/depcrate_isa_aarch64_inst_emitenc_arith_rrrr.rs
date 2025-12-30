// Generated macro for enc_arith_rrrr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_arith_rrrr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_arith_rrrr"}
// Dependencies: {}
fn enc_arith_rrrr (top11 : u32 , rm : Reg , bit15 : u32 , ra : Reg , rn : Reg , rd : Writable < Reg >) -> u32 { (top11 << 21) | (machreg_to_gpr (rm) << 16) | (bit15 << 15) | (machreg_to_gpr (ra) << 10) | (machreg_to_gpr (rn) << 5) | machreg_to_gpr (rd . to_reg ()) }
};
}
