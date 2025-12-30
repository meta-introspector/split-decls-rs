// Generated macro for macro_7158 (macro)
macro_rules! Depcrate_methodsmacro_7158 {
() => {
// Module: crate::methods
// Provides: {"macro_7158"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `FileType::is_file()`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " When people testing a file type with `FileType::is_file`"] # [doc = " they are testing whether a path is something they can get bytes from. But"] # [doc = " `is_file` doesn't cover special file types in unix-like systems, and doesn't cover"] # [doc = " symlink in windows. Using `!FileType::is_dir()` is a better way to that intention."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # || {"] # [doc = " let metadata = std::fs::metadata(\"foo.txt\")?;"] # [doc = " let filetype = metadata.file_type();"] # [doc = ""] # [doc = " if filetype.is_file() {"] # [doc = "     // read file"] # [doc = " }"] # [doc = " # Ok::<_, std::io::Error>(())"] # [doc = " # };"] # [doc = " ```"] # [doc = ""] # [doc = " should be written as:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # || {"] # [doc = " let metadata = std::fs::metadata(\"foo.txt\")?;"] # [doc = " let filetype = metadata.file_type();"] # [doc = ""] # [doc = " if !filetype.is_dir() {"] # [doc = "     // read file"] # [doc = " }"] # [doc = " # Ok::<_, std::io::Error>(())"] # [doc = " # };"] # [doc = " ```"] # [clippy :: version = "1.42.0"] pub FILETYPE_IS_FILE , restriction , "`FileType::is_file` is not recommended to test for readable file type" }
};
}
