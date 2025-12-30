// Generated macro for macro_9641 (macro)
macro_rules! Depcrate_returnsmacro_9641 {
() => {
// Module: crate::returns
// Provides: {"macro_9641"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for return statements at the end of a block."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Removing the `return` and semicolon will make the code"] # [doc = " more rusty."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(x: usize) -> usize {"] # [doc = "     return x;"] # [doc = " }"] # [doc = " ```"] # [doc = " simplify to"] # [doc = " ```no_run"] # [doc = " fn foo(x: usize) -> usize {"] # [doc = "     x"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEEDLESS_RETURN , style , "using a return statement like `return expr;` where an expression would suffice" }
};
}
