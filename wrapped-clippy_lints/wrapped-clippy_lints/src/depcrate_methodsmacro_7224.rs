// Generated macro for macro_7224 (macro)
macro_rules! Depcrate_methodsmacro_7224 {
() => {
// Module: crate::methods
// Provides: {"macro_7224"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `Path::join` that start with a path separator (`\\\\` or `/`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If the argument to `Path::join` starts with a separator, it will overwrite"] # [doc = " the original path. If this is intentional, prefer using `Path::new` instead."] # [doc = ""] # [doc = " Note the behavior is platform dependent. A leading `\\\\` will be accepted"] # [doc = " on unix systems as part of the file name"] # [doc = ""] # [doc = " See [`Path::join`](https://doc.rust-lang.org/std/path/struct.Path.html#method.join)"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " # use std::path::{Path, PathBuf};"] # [doc = " let path = Path::new(\"/bin\");"] # [doc = " let joined_path = path.join(\"/sh\");"] # [doc = " assert_eq!(joined_path, PathBuf::from(\"/sh\"));"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead;"] # [doc = " ```rust"] # [doc = " # use std::path::{Path, PathBuf};"] # [doc = " let path = Path::new(\"/bin\");"] # [doc = ""] # [doc = " // If this was unintentional, remove the leading separator"] # [doc = " let joined_path = path.join(\"sh\");"] # [doc = " assert_eq!(joined_path, PathBuf::from(\"/bin/sh\"));"] # [doc = ""] # [doc = " // If this was intentional, create a new path instead"] # [doc = " let new = Path::new(\"/sh\");"] # [doc = " assert_eq!(new, PathBuf::from(\"/sh\"));"] # [doc = " ```"] # [clippy :: version = "1.76.0"] pub JOIN_ABSOLUTE_PATHS , suspicious , "calls to `Path::join` which will overwrite the original path" }
};
}
