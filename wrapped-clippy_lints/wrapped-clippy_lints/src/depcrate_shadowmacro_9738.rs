// Generated macro for macro_9738 (macro)
macro_rules! Depcrate_shadowmacro_9738 {
() => {
// Module: crate::shadow
// Provides: {"macro_9738"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for bindings that shadow other bindings already in"] # [doc = " scope, while just changing reference level or mutability."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " To require that what are formally distinct variables be given distinct names."] # [doc = ""] # [doc = " See also `shadow_reuse` and `shadow_unrelated` for other restrictions on shadowing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " let x = &x;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " let y = &x; // use different variable name"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SHADOW_SAME , restriction , "rebinding a name to itself, e.g., `let mut x = &mut x`" }
};
}
