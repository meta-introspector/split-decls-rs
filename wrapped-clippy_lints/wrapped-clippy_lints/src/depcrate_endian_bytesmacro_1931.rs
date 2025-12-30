// Generated macro for macro_1931 (macro)
macro_rules! Depcrate_endian_bytesmacro_1931 {
() => {
// Module: crate::endian_bytes
// Provides: {"macro_1931"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the usage of the `to_be_bytes` method and/or the function `from_be_bytes`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " To ensure use of little-endian or the target’s endianness rather than big-endian."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let _x = 2i32.to_be_bytes();"] # [doc = " let _y = 2i64.to_be_bytes();"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub BIG_ENDIAN_BYTES , restriction , "disallows usage of the `to_be_bytes` method" }
};
}
