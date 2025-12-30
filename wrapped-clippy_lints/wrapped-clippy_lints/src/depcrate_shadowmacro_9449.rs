// Generated macro for macro_9449 (macro)
macro_rules! Depcrate_shadowmacro_9449 {
() => {
// Module: crate::shadow
// Provides: {"macro_9449"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for bindings that shadow other bindings already in"] # [doc = " scope, either without an initialization or with one that does not even use"] # [doc = " the original value."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Shadowing a binding with a closely related one is part of idiomatic Rust,"] # [doc = " but shadowing a binding by accident with an unrelated one may indicate a mistake."] # [doc = ""] # [doc = " Additionally, name shadowing in general can hurt readability, especially in"] # [doc = " large code bases, because it is easy to lose track of the active binding at"] # [doc = " any place in the code. If linting against all shadowing is desired, you may wish"] # [doc = " to use the `shadow_same` and `shadow_reuse` lints as well."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let y = 1;"] # [doc = " # let z = 2;"] # [doc = " let x = y;"] # [doc = " let x = z; // shadows the earlier binding"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let y = 1;"] # [doc = " # let z = 2;"] # [doc = " let x = y;"] # [doc = " let w = z; // use different variable name"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SHADOW_UNRELATED , restriction , "rebinding a name without even using the original value" }
};
}
