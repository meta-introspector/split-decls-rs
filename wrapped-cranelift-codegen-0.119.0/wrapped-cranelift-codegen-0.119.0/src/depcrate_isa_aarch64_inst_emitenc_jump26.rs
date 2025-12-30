// Generated macro for enc_jump26 (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_jump26 {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_jump26"}
// Dependencies: {}
fn enc_jump26 (op_31_26 : u32 , off_26_0 : u32) -> u32 { assert ! (off_26_0 < (1 << 26)) ; (op_31_26 << 26) | off_26_0 }
};
}
