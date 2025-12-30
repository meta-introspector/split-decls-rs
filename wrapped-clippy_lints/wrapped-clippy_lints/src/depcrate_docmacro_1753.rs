// Generated macro for macro_1753 (macro)
macro_rules! Depcrate_docmacro_1753 {
() => {
// Module: crate::doc
// Provides: {"macro_1753"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects documentation that is empty."] # [doc = " ### Why is this bad?"] # [doc = " Empty docs clutter code without adding value, reducing readability and maintainability."] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " ///"] # [doc = " fn returns_true() -> bool {"] # [doc = "     true"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn returns_true() -> bool {"] # [doc = "     true"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub EMPTY_DOCS , suspicious , "docstrings exist but documentation is empty" }
};
}
