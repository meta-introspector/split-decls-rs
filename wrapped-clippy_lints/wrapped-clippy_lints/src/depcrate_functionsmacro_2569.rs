// Generated macro for macro_2569 (macro)
macro_rules! Depcrate_functionsmacro_2569 {
() => {
// Module: crate::functions
// Provides: {"macro_2569"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for function arguments having the similar names"] # [doc = " differing by an underscore."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It affects code readability."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(a: i32, _a: i32) {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn bar(a: i32, _b: i32) {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub DUPLICATE_UNDERSCORE_ARGUMENT , style , "function arguments having names which only differ by an underscore" }
};
}
