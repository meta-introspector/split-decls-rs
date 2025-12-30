// Generated macro for default_len (function)
macro_rules! Depcrate_shims_x86_sse42default_len {
() => {
// Module: crate::shims::x86::sse42
// Provides: {"default_len"}
// Dependencies: {}
# [inline] fn default_len < T : From < u8 > > (imm : u8) -> T { if imm & USE_WORDS != 0 { T :: from (8u8) } else { T :: from (16u8) } }
};
}
