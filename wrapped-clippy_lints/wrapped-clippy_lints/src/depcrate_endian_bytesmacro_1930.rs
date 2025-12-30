// Generated macro for macro_1930 (macro)
macro_rules! Depcrate_endian_bytesmacro_1930 {
() => {
// Module: crate::endian_bytes
// Provides: {"macro_1930"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the usage of the `to_le_bytes` method and/or the function `from_le_bytes`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " To ensure use of big-endian or the target’s endianness rather than little-endian."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let _x = 2i32.to_le_bytes();"] # [doc = " let _y = 2i64.to_le_bytes();"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub LITTLE_ENDIAN_BYTES , restriction , "disallows usage of the `to_le_bytes` method" }
};
}
