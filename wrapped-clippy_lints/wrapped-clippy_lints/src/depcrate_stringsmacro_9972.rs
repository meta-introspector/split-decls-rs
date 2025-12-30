// Generated macro for macro_9972 (macro)
macro_rules! Depcrate_stringsmacro_9972 {
() => {
// Module: crate::strings
// Provides: {"macro_9972"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for all instances of `x + _` where `x` is of type"] # [doc = " `String`, but only if [`string_add_assign`](#string_add_assign) does *not*"] # [doc = " match."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " This particular"] # [doc = " `Add` implementation is asymmetric (the other operand need not be `String`,"] # [doc = " but `x` does), while addition as mathematically defined is symmetric, and"] # [doc = " the `String::push_str(_)` function is a perfectly good replacement."] # [doc = " Therefore, some dislike it and wish not to have it in their code."] # [doc = ""] # [doc = " That said, other people think that string addition, having a long tradition"] # [doc = " in other languages is actually fine, which is why we decided to make this"] # [doc = " particular lint `allow` by default."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = \"Hello\".to_owned();"] # [doc = " x + \", World\";"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut x = \"Hello\".to_owned();"] # [doc = " x.push_str(\", World\");"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub STRING_ADD , restriction , "using `x + ..` where x is a `String` instead of `push_str()`" }
};
}
