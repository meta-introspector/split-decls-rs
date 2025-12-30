// Generated macro for macro_7180 (macro)
macro_rules! Depcrate_methodsmacro_7180 {
() => {
// Module: crate::methods
// Provides: {"macro_7180"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calling `take` function after `as_ref`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Redundant code. `take` writes `None` to its argument."] # [doc = " In this case the modification is useless as it's a temporary that cannot be read from afterwards."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = Some(3);"] # [doc = " x.as_ref().take();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = Some(3);"] # [doc = " x.as_ref();"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub NEEDLESS_OPTION_TAKE , complexity , "using `.as_ref().take()` on a temporary value" }
};
}
