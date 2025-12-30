// Generated macro for enc_cbr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_cbr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_cbr"}
// Dependencies: {}
fn enc_cbr (op_31_24 : u32 , off_18_0 : u32 , op_4 : u32 , cond : u32) -> u32 { assert ! (off_18_0 < (1 << 19)) ; assert ! (cond < (1 << 4)) ; (op_31_24 << 24) | (off_18_0 << 5) | (op_4 << 4) | cond }
};
}
