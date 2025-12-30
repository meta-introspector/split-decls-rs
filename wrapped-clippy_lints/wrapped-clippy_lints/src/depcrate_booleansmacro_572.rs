// Generated macro for macro_572 (macro)
macro_rules! Depcrate_booleansmacro_572 {
() => {
// Module: crate::booleans
// Provides: {"macro_572"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for boolean expressions that can be written more"] # [doc = " concisely."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability of boolean expressions suffers from"] # [doc = " unnecessary duplication."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Ignores short circuiting behavior of `||` and"] # [doc = " `&&`. Ignores `|`, `&` and `^`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " if a && true {}"] # [doc = " if !(a == b) {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " if a {}"] # [doc = " if a != b {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NONMINIMAL_BOOL , complexity , "boolean expressions that can be written more concisely" }
};
}
