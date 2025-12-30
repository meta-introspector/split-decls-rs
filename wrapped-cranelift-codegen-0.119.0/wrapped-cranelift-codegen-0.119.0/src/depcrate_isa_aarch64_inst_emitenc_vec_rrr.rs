// Generated macro for enc_vec_rrr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_vec_rrr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_vec_rrr"}
// Dependencies: {}
fn enc_vec_rrr (top11 : u32 , rm : Reg , bit15_10 : u32 , rn : Reg , rd : Writable < Reg >) -> u32 { (top11 << 21) | (machreg_to_vec (rm) << 16) | (bit15_10 << 10) | (machreg_to_vec (rn) << 5) | machreg_to_vec (rd . to_reg ()) }
};
}
