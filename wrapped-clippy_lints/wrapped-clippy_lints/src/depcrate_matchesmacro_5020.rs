// Generated macro for macro_5020 (macro)
macro_rules! Depcrate_matchesmacro_5020 {
() => {
// Module: crate::matches
// Provides: {"macro_5020"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for matches where match expression is a `bool`. It"] # [doc = " suggests to replace the expression with an `if...else` block."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It makes the code less readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # fn foo() {}"] # [doc = " # fn bar() {}"] # [doc = " let condition: bool = true;"] # [doc = " match condition {"] # [doc = "     true => foo(),"] # [doc = "     false => bar(),"] # [doc = " }"] # [doc = " ```"] # [doc = " Use if/else instead:"] # [doc = " ```no_run"] # [doc = " # fn foo() {}"] # [doc = " # fn bar() {}"] # [doc = " let condition: bool = true;"] # [doc = " if condition {"] # [doc = "     foo();"] # [doc = " } else {"] # [doc = "     bar();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MATCH_BOOL , pedantic , "a `match` on a boolean expression instead of an `if..else` block" }
};
}
