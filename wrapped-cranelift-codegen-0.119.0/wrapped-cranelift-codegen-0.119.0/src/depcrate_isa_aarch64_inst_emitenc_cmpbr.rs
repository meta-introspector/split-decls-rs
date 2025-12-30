// Generated macro for enc_cmpbr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_cmpbr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_cmpbr"}
// Dependencies: {}
fn enc_cmpbr (op_31_24 : u32 , off_18_0 : u32 , reg : Reg) -> u32 { assert ! (off_18_0 < (1 << 19)) ; (op_31_24 << 24) | (off_18_0 << 5) | machreg_to_gpr (reg) }
};
}
