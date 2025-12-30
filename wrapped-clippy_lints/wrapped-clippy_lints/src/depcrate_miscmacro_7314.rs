// Generated macro for macro_7314 (macro)
macro_rules! Depcrate_miscmacro_7314 {
() => {
// Module: crate::misc
// Provides: {"macro_7314"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the use of bindings with a single leading"] # [doc = " underscore."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " A single leading underscore is usually used to indicate"] # [doc = " that a binding will not be used. Using such a binding breaks this"] # [doc = " expectation."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The lint does not work properly with desugaring and"] # [doc = " macro, it has been allowed in the meantime."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _x = 0;"] # [doc = " let y = _x + 1; // Here we are using `_x`, even though it has a leading"] # [doc = "                 // underscore. We should rename `_x` to `x`"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub USED_UNDERSCORE_BINDING , pedantic , "using a binding which is prefixed with an underscore" }
};
}
