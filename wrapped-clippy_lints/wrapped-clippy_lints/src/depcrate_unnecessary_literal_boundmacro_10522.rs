// Generated macro for macro_10522 (macro)
macro_rules! Depcrate_unnecessary_literal_boundmacro_10522 {
() => {
// Module: crate::unnecessary_literal_bound
// Provides: {"macro_10522"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Detects functions that are written to return `&str` that could return `&'static str` but instead return a `&'a str`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " This leaves the caller unable to use the `&str` as `&'static str`, causing unnecessary allocations or confusion."] # [doc = " This is also most likely what you meant to write."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # struct MyType;"] # [doc = " impl MyType {"] # [doc = "     fn returns_literal(&self) -> &str {"] # [doc = "         \"Literal\""] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # struct MyType;"] # [doc = " impl MyType {"] # [doc = "     fn returns_literal(&self) -> &'static str {"] # [doc = "         \"Literal\""] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Or, in case you may return a non-literal `str` in future:"] # [doc = " ```no_run"] # [doc = " # struct MyType;"] # [doc = " impl MyType {"] # [doc = "     fn returns_literal<'a>(&'a self) -> &'a str {"] # [doc = "         \"Literal\""] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.84.0"] pub UNNECESSARY_LITERAL_BOUND , pedantic , "detects &str that could be &'static str in function return types" }
};
}
