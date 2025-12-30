// Generated macro for macro_5042 (macro)
macro_rules! Depcrate_matchesmacro_5042 {
() => {
// Module: crate::matches
// Provides: {"macro_5042"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unnecessary guards in match expressions."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's more complex and much less readable. Making it part of the pattern can improve"] # [doc = " exhaustiveness checking as well."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " match x {"] # [doc = "     Some(x) if matches!(x, Some(1)) => ..,"] # [doc = "     Some(x) if x == Some(2) => ..,"] # [doc = "     _ => todo!(),"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " match x {"] # [doc = "     Some(Some(1)) => ..,"] # [doc = "     Some(Some(2)) => ..,"] # [doc = "     _ => todo!(),"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub REDUNDANT_GUARDS , complexity , "checks for unnecessary guards in match expressions" }
};
}
