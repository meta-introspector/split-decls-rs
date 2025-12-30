// Generated macro for enc_inttofpu (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_inttofpu {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_inttofpu"}
// Dependencies: {}
fn enc_inttofpu (top16 : u32 , rd : Writable < Reg > , rn : Reg) -> u32 { (top16 << 16) | (machreg_to_gpr (rn) << 5) | machreg_to_vec (rd . to_reg ()) }
};
}
