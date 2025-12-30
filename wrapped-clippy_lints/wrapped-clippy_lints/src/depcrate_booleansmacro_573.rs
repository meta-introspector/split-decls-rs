// Generated macro for macro_573 (macro)
macro_rules! Depcrate_booleansmacro_573 {
() => {
// Module: crate::booleans
// Provides: {"macro_573"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for boolean expressions that contain terminals that"] # [doc = " can be eliminated."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is most likely a logic bug."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Ignores short circuiting behavior."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " // The `b` is unnecessary, the expression is equivalent to `if a`."] # [doc = " if a && b || a { ... }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " if a {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub OVERLY_COMPLEX_BOOL_EXPR , correctness , "boolean expressions that contain terminals which can be eliminated" }
};
}
