// Generated macro for macro_7047 (macro)
macro_rules! Depcrate_methodsmacro_7047 {
() => {
// Module: crate::methods
// Provides: {"macro_7047"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = "* Checks for [push](https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.push)"] # [doc = " calls on `PathBuf` that can cause overwrites."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Calling `push` with a root path at the start can overwrite the"] # [doc = " previous defined path."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::path::PathBuf;"] # [doc = ""] # [doc = " let mut x = PathBuf::from(\"/foo\");"] # [doc = " x.push(\"/bar\");"] # [doc = " assert_eq!(x, PathBuf::from(\"/bar\"));"] # [doc = " ```"] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::path::PathBuf;"] # [doc = ""] # [doc = " let mut x = PathBuf::from(\"/foo\");"] # [doc = " x.push(\"bar\");"] # [doc = " assert_eq!(x, PathBuf::from(\"/foo/bar\"));"] # [doc = " ```"] # [clippy :: version = "1.36.0"] pub PATH_BUF_PUSH_OVERWRITE , nursery , "calling `push` with file system root on `PathBuf` can overwrite it" }
};
}
