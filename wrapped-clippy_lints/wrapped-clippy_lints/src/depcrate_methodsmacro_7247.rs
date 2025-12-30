// Generated macro for macro_7247 (macro)
macro_rules! Depcrate_methodsmacro_7247 {
() => {
// Module: crate::methods
// Provides: {"macro_7247"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detect functions that end with `Option::and_then` or `Result::and_then`, and suggest using"] # [doc = " the `?` operator instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `and_then` method is used to chain a computation that returns an `Option` or a `Result`."] # [doc = " This can be replaced with the `?` operator, which is more concise and idiomatic."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " fn test(opt: Option<i32>) -> Option<i32> {"] # [doc = "     opt.and_then(|n| {"] # [doc = "         if n > 1 {"] # [doc = "             Some(n + 1)"] # [doc = "         } else {"] # [doc = "             None"] # [doc = "        }"] # [doc = "     })"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn test(opt: Option<i32>) -> Option<i32> {"] # [doc = "     let n = opt?;"] # [doc = "     if n > 1 {"] # [doc = "         Some(n + 1)"] # [doc = "     } else {"] # [doc = "         None"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub RETURN_AND_THEN , restriction , "using `Option::and_then` or `Result::and_then` to chain a computation that returns an `Option` or a `Result`" }
};
}
