// Generated macro for enc_fputoint (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_fputoint {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_fputoint"}
// Dependencies: {}
fn enc_fputoint (top16 : u32 , rd : Writable < Reg > , rn : Reg) -> u32 { (top16 << 16) | (machreg_to_vec (rn) << 5) | machreg_to_gpr (rd . to_reg ()) }
};
}
