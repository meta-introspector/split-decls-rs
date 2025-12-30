// Generated macro for macro_8088 (macro)
macro_rules! Depcrate_no_effectmacro_8088 {
() => {
// Module: crate::no_effect
// Provides: {"macro_8088"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for statements which have no effect."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Unlike dead code, these statements are actually"] # [doc = " executed. However, as they have no effect, all they do is make the code less"] # [doc = " readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " 0;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NO_EFFECT , complexity , "statements with no effect" }
};
}
