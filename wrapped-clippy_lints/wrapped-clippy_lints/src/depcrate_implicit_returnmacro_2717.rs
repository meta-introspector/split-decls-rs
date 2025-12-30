// Generated macro for macro_2717 (macro)
macro_rules! Depcrate_implicit_returnmacro_2717 {
() => {
// Module: crate::implicit_return
// Provides: {"macro_2717"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for missing return statements at the end of a block."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Omitting the return keyword whenever possible is idiomatic Rust code, but:"] # [doc = ""] # [doc = " * Programmers coming from other languages might prefer the expressiveness of `return`."] # [doc = " * It's possible to miss the last returning statement because the only difference is a missing `;`."] # [doc = " * Especially in bigger code with multiple return paths, having a `return` keyword makes it easier to find the"] # [doc = "   corresponding statements."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(x: usize) -> usize {"] # [doc = "     x"] # [doc = " }"] # [doc = " ```"] # [doc = " add return"] # [doc = " ```no_run"] # [doc = " fn foo(x: usize) -> usize {"] # [doc = "     return x;"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.33.0"] pub IMPLICIT_RETURN , restriction , "use a return statement like `return expr` instead of an expression" }
};
}
