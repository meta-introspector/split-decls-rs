// Generated macro for macro_7030 (macro)
macro_rules! Depcrate_methodsmacro_7030 {
() => {
// Module: crate::methods
// Provides: {"macro_7030"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for no-op uses of `Option::{as_deref, as_deref_mut}`,"] # [doc = " for example, `Option<&T>::as_deref()` returns the same type."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Redundant code and improving readability."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a = Some(&1);"] # [doc = " let b = a.as_deref(); // goes from Option<&i32> to Option<&i32>"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a = Some(&1);"] # [doc = " let b = a;"] # [doc = " ```"] # [clippy :: version = "1.57.0"] pub NEEDLESS_OPTION_AS_DEREF , complexity , "no-op use of `deref` or `deref_mut` method to `Option`." }
};
}
