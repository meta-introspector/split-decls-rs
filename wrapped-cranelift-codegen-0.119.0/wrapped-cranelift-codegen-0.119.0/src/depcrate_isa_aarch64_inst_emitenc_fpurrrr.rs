// Generated macro for enc_fpurrrr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_fpurrrr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_fpurrrr"}
// Dependencies: {}
fn enc_fpurrrr (top17 : u32 , rd : Writable < Reg > , rn : Reg , rm : Reg , ra : Reg) -> u32 { (top17 << 15) | (machreg_to_vec (rm) << 16) | (machreg_to_vec (ra) << 10) | (machreg_to_vec (rn) << 5) | machreg_to_vec (rd . to_reg ()) }
};
}
