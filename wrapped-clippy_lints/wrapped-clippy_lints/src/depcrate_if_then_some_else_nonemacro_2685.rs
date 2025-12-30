// Generated macro for macro_2685 (macro)
macro_rules! Depcrate_if_then_some_else_nonemacro_2685 {
() => {
// Module: crate::if_then_some_else_none
// Provides: {"macro_2685"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for if-else that could be written using either `bool::then` or `bool::then_some`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Looks a little redundant. Using `bool::then` is more concise and incurs no loss of clarity."] # [doc = " For simple calculations and known values, use `bool::then_some`, which is eagerly evaluated"] # [doc = " in comparison to `bool::then`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let v = vec![0];"] # [doc = " let a = if v.is_empty() {"] # [doc = "     println!(\"true!\");"] # [doc = "     Some(42)"] # [doc = " } else {"] # [doc = "     None"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let v = vec![0];"] # [doc = " let a = v.is_empty().then(|| {"] # [doc = "     println!(\"true!\");"] # [doc = "     42"] # [doc = " });"] # [doc = " ```"] # [clippy :: version = "1.53.0"] pub IF_THEN_SOME_ELSE_NONE , restriction , "Finds if-else that could be written using either `bool::then` or `bool::then_some`" }
};
}
