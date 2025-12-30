// Generated macro for macro_7999 (macro)
macro_rules! Depcrate_needless_pass_by_valuemacro_7999 {
() => {
// Module: crate::needless_pass_by_value
// Provides: {"macro_7999"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for functions taking arguments by value, but not"] # [doc = " consuming them in its"] # [doc = " body."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Taking arguments by reference is more flexible and can"] # [doc = " sometimes avoid"] # [doc = " unnecessary allocations."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " * This lint suggests taking an argument by reference,"] # [doc = " however sometimes it is better to let users decide the argument type"] # [doc = " (by using `Borrow` trait, for example), depending on how the function is used."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(v: Vec<i32>) {"] # [doc = "     assert_eq!(v.len(), 42);"] # [doc = " }"] # [doc = " ```"] # [doc = " should be"] # [doc = " ```no_run"] # [doc = " fn foo(v: &[i32]) {"] # [doc = "     assert_eq!(v.len(), 42);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEEDLESS_PASS_BY_VALUE , pedantic , "functions taking arguments by value, but not consuming them in its body" }
};
}
