// Generated macro for macro_4982 (macro)
macro_rules! Depcrate_matchesmacro_4982 {
() => {
// Module: crate::matches
// Provides: {"macro_4982"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `match` with identical arm bodies."] # [doc = ""] # [doc = " Note: Does not lint on wildcards if the `non_exhaustive_omitted_patterns_lint` feature is"] # [doc = " enabled and disallowed."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is probably a copy & paste error. If arm bodies"] # [doc = " are the same on purpose, you can factor them"] # [doc = " [using `|`](https://doc.rust-lang.org/book/patterns.html#multiple-patterns)."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " match foo {"] # [doc = "     Bar => bar(),"] # [doc = "     Quz => quz(),"] # [doc = "     Baz => bar(), // <= oops"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This should probably be"] # [doc = " ```rust,ignore"] # [doc = " match foo {"] # [doc = "     Bar => bar(),"] # [doc = "     Quz => quz(),"] # [doc = "     Baz => baz(), // <= fixed"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " or if the original code was not a typo:"] # [doc = " ```rust,ignore"] # [doc = " match foo {"] # [doc = "     Bar | Baz => bar(), // <= shows the intent better"] # [doc = "     Quz => quz(),"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MATCH_SAME_ARMS , pedantic , "`match` with identical arm bodies" }
};
}
