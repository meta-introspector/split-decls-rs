// Generated macro for macro_7001 (macro)
macro_rules! Depcrate_methodsmacro_7001 {
() => {
// Module: crate::methods
// Provides: {"macro_7001"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.as_ref()` or `.as_mut()` where the"] # [doc = " types before and after the call are the same."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The call is unnecessary."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # fn do_stuff(x: &[i32]) {}"] # [doc = " let x: &[i32] = &[1, 2, 3, 4, 5];"] # [doc = " do_stuff(x.as_ref());"] # [doc = " ```"] # [doc = " The correct use would be:"] # [doc = " ```no_run"] # [doc = " # fn do_stuff(x: &[i32]) {}"] # [doc = " let x: &[i32] = &[1, 2, 3, 4, 5];"] # [doc = " do_stuff(x);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub USELESS_ASREF , complexity , "using `as_ref` where the types before and after the call are the same" }
};
}
