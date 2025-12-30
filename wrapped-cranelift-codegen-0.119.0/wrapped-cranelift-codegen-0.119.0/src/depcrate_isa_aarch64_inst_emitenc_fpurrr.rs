// Generated macro for enc_fpurrr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_fpurrr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_fpurrr"}
// Dependencies: {}
fn enc_fpurrr (top22 : u32 , rd : Writable < Reg > , rn : Reg , rm : Reg) -> u32 { (top22 << 10) | (machreg_to_vec (rm) << 16) | (machreg_to_vec (rn) << 5) | machreg_to_vec (rd . to_reg ()) }
};
}
