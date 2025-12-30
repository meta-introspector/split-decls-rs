// Generated macro for macro_4969 (macro)
macro_rules! Depcrate_matchesmacro_4969 {
() => {
// Module: crate::matches
// Provides: {"macro_4969"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for matches where all arms match a reference,"] # [doc = " suggesting to remove the reference and deref the matched expression"] # [doc = " instead. It also checks for `if let &foo = bar` blocks."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It just makes the code less readable. That reference"] # [doc = " destructuring adds nothing to the code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " match x {"] # [doc = "     &A(ref y) => foo(y),"] # [doc = "     &B => bar(),"] # [doc = "     _ => frob(&x),"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " match *x {"] # [doc = "     A(ref y) => foo(y),"] # [doc = "     B => bar(),"] # [doc = "     _ => frob(x),"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MATCH_REF_PATS , style , "a `match` or `if let` with all arms prefixed with `&` instead of deref-ing the match expression" }
};
}
