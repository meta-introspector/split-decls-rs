// Generated macro for enc_fround (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_fround {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_fround"}
// Dependencies: {}
fn enc_fround (top22 : u32 , rd : Writable < Reg > , rn : Reg) -> u32 { (top22 << 10) | (machreg_to_vec (rn) << 5) | machreg_to_vec (rd . to_reg ()) }
};
}
