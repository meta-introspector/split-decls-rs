// Generated macro for macro_7092 (macro)
macro_rules! Depcrate_methodsmacro_7092 {
() => {
// Module: crate::methods
// Provides: {"macro_7092"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " It detects useless calls to `str::as_bytes()` before calling `len()` or `is_empty()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `len()` and `is_empty()` methods are also directly available on strings, and they"] # [doc = " return identical results. In particular, `len()` on a string returns the number of"] # [doc = " bytes."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```"] # [doc = " let len = \"some string\".as_bytes().len();"] # [doc = " let b = \"some string\".as_bytes().is_empty();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```"] # [doc = " let len = \"some string\".len();"] # [doc = " let b = \"some string\".is_empty();"] # [doc = " ```"] # [clippy :: version = "1.84.0"] pub NEEDLESS_AS_BYTES , complexity , "detect useless calls to `as_bytes()`" }
};
}
