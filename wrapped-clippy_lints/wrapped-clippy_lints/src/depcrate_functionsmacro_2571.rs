// Generated macro for macro_2571 (macro)
macro_rules! Depcrate_functionsmacro_2571 {
() => {
// Module: crate::functions
// Provides: {"macro_2571"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for functions with a large amount of lines."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Functions with a lot of lines are harder to understand"] # [doc = " due to having to look at a larger amount of code to understand what the"] # [doc = " function is doing. Consider splitting the body of the function into"] # [doc = " multiple functions."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn im_too_long() {"] # [doc = "     println!(\"\");"] # [doc = "     // ... 100 more LoC"] # [doc = "     println!(\"\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.34.0"] pub TOO_MANY_LINES , pedantic , "functions with too many lines" }
};
}
