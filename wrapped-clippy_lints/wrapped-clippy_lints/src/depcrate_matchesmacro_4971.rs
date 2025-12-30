// Generated macro for macro_4971 (macro)
macro_rules! Depcrate_matchesmacro_4971 {
() => {
// Module: crate::matches
// Provides: {"macro_4971"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for overlapping match arms."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is likely to be an error and if not, makes the code"] # [doc = " less obvious."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = 5;"] # [doc = " match x {"] # [doc = "     1..=10 => println!(\"1 ... 10\"),"] # [doc = "     5..=15 => println!(\"5 ... 15\"),"] # [doc = "     _ => (),"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MATCH_OVERLAPPING_ARM , style , "a `match` with overlapping arms" }
};
}
