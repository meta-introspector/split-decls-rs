// Generated macro for macro_9675 (macro)
macro_rules! Depcrate_stringsmacro_9675 {
() => {
// Module: crate::strings
// Provides: {"macro_9675"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for string appends of the form `x = x + y` (without"] # [doc = " `let`!)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's not really bad, but some people think that the"] # [doc = " `.push_str(_)` method is more readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut x = \"Hello\".to_owned();"] # [doc = " x = x + \", World\";"] # [doc = ""] # [doc = " // More readable"] # [doc = " x += \", World\";"] # [doc = " x.push_str(\", World\");"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub STRING_ADD_ASSIGN , pedantic , "using `x = x + ..` where x is a `String` instead of `push_str()`" }
};
}
