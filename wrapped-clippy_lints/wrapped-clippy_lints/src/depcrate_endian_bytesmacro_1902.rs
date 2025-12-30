// Generated macro for macro_1902 (macro)
macro_rules! Depcrate_endian_bytesmacro_1902 {
() => {
// Module: crate::endian_bytes
// Provides: {"macro_1902"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the usage of the `to_ne_bytes` method and/or the function `from_ne_bytes`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " To ensure use of explicitly chosen endianness rather than the target’s endianness,"] # [doc = " such as when implementing network protocols or file formats rather than FFI."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let _x = 2i32.to_ne_bytes();"] # [doc = " let _y = 2i64.to_ne_bytes();"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub HOST_ENDIAN_BYTES , restriction , "disallows usage of the `to_ne_bytes` method" }
};
}
