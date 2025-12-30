// Generated macro for macro_4967 (macro)
macro_rules! Depcrate_matchesmacro_4967 {
() => {
// Module: crate::matches
// Provides: {"macro_4967"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for matches with a single arm where an `if let`"] # [doc = " will usually suffice."] # [doc = ""] # [doc = " This intentionally does not lint if there are comments"] # [doc = " inside of the other arm, so as to allow the user to document"] # [doc = " why having another explicit pattern with an empty body is necessary,"] # [doc = " or because the comments need to be preserved for other reasons."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Just readability – `if let` nests less than a `match`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # fn bar(stool: &str) {}"] # [doc = " # let x = Some(\"abc\");"] # [doc = " match x {"] # [doc = "     Some(ref foo) => bar(foo),"] # [doc = "     _ => (),"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # fn bar(stool: &str) {}"] # [doc = " # let x = Some(\"abc\");"] # [doc = " if let Some(ref foo) = x {"] # [doc = "     bar(foo);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SINGLE_MATCH , style , "a `match` statement with a single nontrivial arm (i.e., where the other arm is `_ => {}`) instead of `if let`" }
};
}
