// Generated macro for macro_7168 (macro)
macro_rules! Depcrate_methodsmacro_7168 {
() => {
// Module: crate::methods
// Provides: {"macro_7168"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the use of `.bytes().nth()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `.as_bytes().get()` is more efficient and more"] # [doc = " readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " \"Hello\".bytes().nth(3);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " \"Hello\".as_bytes().get(3);"] # [doc = " ```"] # [clippy :: version = "1.52.0"] pub BYTES_NTH , style , "replace `.bytes().nth()` with `.as_bytes().get()`" }
};
}
