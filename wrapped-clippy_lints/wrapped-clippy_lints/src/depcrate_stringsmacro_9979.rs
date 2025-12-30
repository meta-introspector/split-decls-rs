// Generated macro for macro_9979 (macro)
macro_rules! Depcrate_stringsmacro_9979 {
() => {
// Module: crate::strings
// Provides: {"macro_9979"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Check if the string is transformed to byte array and casted back to string."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's unnecessary, the string can be used directly."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " std::str::from_utf8(&\"Hello World!\".as_bytes()[6..11]).unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " &\"Hello World!\"[6..11];"] # [doc = " ```"] # [clippy :: version = "1.50.0"] pub STRING_FROM_UTF8_AS_BYTES , complexity , "casting string slices to byte slices and back" }
};
}
