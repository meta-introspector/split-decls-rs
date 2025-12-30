// Generated macro for macro_9341 (macro)
macro_rules! Depcrate_returnsmacro_9341 {
() => {
// Module: crate::returns
// Provides: {"macro_9341"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `let`-bindings, which are subsequently"] # [doc = " returned."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is just extraneous code. Remove it to make your code"] # [doc = " more rusty."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " In the case of some temporaries, e.g. locks, eliding the variable binding could lead"] # [doc = " to deadlocks. See [this issue](https://github.com/rust-lang/rust/issues/37612)."] # [doc = " This could become relevant if the code is later changed to use the code that would have been"] # [doc = " bound without first assigning it to a let-binding."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo() -> String {"] # [doc = "     let x = String::new();"] # [doc = "     x"] # [doc = " }"] # [doc = " ```"] # [doc = " instead, use"] # [doc = " ```no_run"] # [doc = " fn foo() -> String {"] # [doc = "     String::new()"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub LET_AND_RETURN , style , "creating a let-binding and then immediately returning it like `let x = expr; x` at the end of a block" }
};
}
