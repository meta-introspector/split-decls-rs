// Generated macro for macro_7942 (macro)
macro_rules! Depcrate_no_effectmacro_7942 {
() => {
// Module: crate::no_effect
// Provides: {"macro_7942"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for binding to underscore prefixed variable without side-effects."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Unlike dead code, these bindings are actually"] # [doc = " executed. However, as they have no effect and shouldn't be used further on, all they"] # [doc = " do is make the code less readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let _i_serve_no_purpose = 1;"] # [doc = " ```"] # [clippy :: version = "1.58.0"] pub NO_EFFECT_UNDERSCORE_BINDING , pedantic , "binding to `_` prefixed variable with no side-effect" }
};
}
