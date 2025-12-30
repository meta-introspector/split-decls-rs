// Generated macro for macro_2379 (macro)
macro_rules! Depcrate_four_forward_slashesmacro_2379 {
() => {
// Module: crate::four_forward_slashes
// Provides: {"macro_2379"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for outer doc comments written with 4 forward slashes (`////`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is (probably) a typo, and results in it not being a doc comment; just a regular"] # [doc = " comment."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " //// My amazing data structure"] # [doc = " pub struct Foo {"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " /// My amazing data structure"] # [doc = " pub struct Foo {"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub FOUR_FORWARD_SLASHES , suspicious , "comments with 4 forward slashes (`////`) likely intended to be doc comments (`///`)" }
};
}
