// Generated macro for macro_9739 (macro)
macro_rules! Depcrate_shadowmacro_9739 {
() => {
// Module: crate::shadow
// Provides: {"macro_9739"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for bindings that shadow other bindings already in"] # [doc = " scope, while reusing the original value."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Some argue that name shadowing like this hurts readability,"] # [doc = " because a value may be bound to different things depending on position in"] # [doc = " the code."] # [doc = ""] # [doc = " See also `shadow_same` and `shadow_unrelated` for other restrictions on shadowing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = 2;"] # [doc = " let x = x + 1;"] # [doc = " ```"] # [doc = " use different variable name:"] # [doc = " ```no_run"] # [doc = " let x = 2;"] # [doc = " let y = x + 1;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SHADOW_REUSE , restriction , "rebinding a name to an expression that reuses the original value, e.g., `let x = x + 1`" }
};
}
