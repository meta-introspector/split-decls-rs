// Generated macro for macro_1768 (macro)
macro_rules! Depcrate_docmacro_1768 {
() => {
// Module: crate::doc
// Provides: {"macro_1768"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects the syntax `['foo']` in documentation comments (notice quotes instead of backticks)"] # [doc = " outside of code blocks"] # [doc = " ### Why is this bad?"] # [doc = " It is likely a typo when defining an intra-doc link"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " /// See also: ['foo']"] # [doc = " fn bar() {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " /// See also: [`foo`]"] # [doc = " fn bar() {}"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub DOC_LINK_WITH_QUOTES , pedantic , "possible typo for an intra-doc link" }
};
}
