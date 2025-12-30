// Generated macro for U32x2 (struct)
macro_rules! Depcrate_rngU32x2 {
() => {
// Module: crate::rng
// Provides: {"U32x2"}
// Dependencies: {}
# [doc = " A wrapper around 64 bits of data that can be constructed from any of the"] # [doc = " following:"] # [doc = " * `u64`"] # [doc = " * `[u32; 2]`"] # [doc = " * `[u8; 8]`"] # [doc = ""] # [doc = " The arrays should be in little endian order. You should not need to use"] # [doc = " this directly, as the methods in this crate that use this type call"] # [doc = " `.into()` for you, so you only need to supply any of the above types."] pub struct U32x2 ([u32 ; Self :: LEN]) ;
};
}
