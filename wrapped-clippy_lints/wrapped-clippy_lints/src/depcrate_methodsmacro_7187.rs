// Generated macro for macro_7187 (macro)
macro_rules! Depcrate_methodsmacro_7187 {
() => {
// Module: crate::methods
// Provides: {"macro_7187"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `ends_with` with possible file extensions"] # [doc = " and suggests to use a case-insensitive approach instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `ends_with` is case-sensitive and may not detect files with a valid extension."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn is_rust_file(filename: &str) -> bool {"] # [doc = "     filename.ends_with(\".rs\")"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn is_rust_file(filename: &str) -> bool {"] # [doc = "     let filename = std::path::Path::new(filename);"] # [doc = "     filename.extension()"] # [doc = "         .map_or(false, |ext| ext.eq_ignore_ascii_case(\"rs\"))"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub CASE_SENSITIVE_FILE_EXTENSION_COMPARISONS , pedantic , "Checks for calls to ends_with with case-sensitive file extensions" }
};
}
