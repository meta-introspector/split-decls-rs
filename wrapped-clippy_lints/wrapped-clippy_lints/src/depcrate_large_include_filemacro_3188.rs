// Generated macro for macro_3188 (macro)
macro_rules! Depcrate_large_include_filemacro_3188 {
() => {
// Module: crate::large_include_file
// Provides: {"macro_3188"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the inclusion of large files via `include_bytes!()`"] # [doc = " or `include_str!()`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Including large files can undesirably increase the size of the binary produced by the compiler."] # [doc = " This lint may be used to catch mistakes where an unexpectedly large file is included, or"] # [doc = " temporarily to obtain a list of all large files."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let included_str = include_str!(\"very_large_file.txt\");"] # [doc = " let included_bytes = include_bytes!(\"very_large_file.txt\");"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " use std::fs;"] # [doc = ""] # [doc = " // You can load the file at runtime"] # [doc = " let string = fs::read_to_string(\"very_large_file.txt\")?;"] # [doc = " let bytes = fs::read(\"very_large_file.txt\")?;"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub LARGE_INCLUDE_FILE , restriction , "including a large file" }
};
}
