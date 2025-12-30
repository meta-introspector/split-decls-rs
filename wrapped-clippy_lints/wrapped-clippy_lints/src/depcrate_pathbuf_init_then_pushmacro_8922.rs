// Generated macro for macro_8922 (macro)
macro_rules! Depcrate_pathbuf_init_then_pushmacro_8922 {
() => {
// Module: crate::pathbuf_init_then_push
// Provides: {"macro_8922"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `push` immediately after creating a new `PathBuf`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Multiple `.join()` calls are usually easier to read than multiple `.push`"] # [doc = " calls across multiple statements. It might also be possible to use"] # [doc = " `PathBuf::from` instead."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " `.join()` introduces an implicit `clone()`. `PathBuf::from` can alternatively be"] # [doc = " used when the `PathBuf` is newly constructed. This will avoid the implicit clone."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " # use std::path::PathBuf;"] # [doc = " let mut path_buf = PathBuf::new();"] # [doc = " path_buf.push(\"foo\");"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust"] # [doc = " # use std::path::PathBuf;"] # [doc = " let path_buf = PathBuf::from(\"foo\");"] # [doc = " // or"] # [doc = " let path_buf = PathBuf::new().join(\"foo\");"] # [doc = " ```"] # [clippy :: version = "1.82.0"] pub PATHBUF_INIT_THEN_PUSH , restriction , "`push` immediately after `PathBuf` creation" }
};
}
