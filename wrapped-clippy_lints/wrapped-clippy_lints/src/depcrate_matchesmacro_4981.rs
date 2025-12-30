// Generated macro for macro_4981 (macro)
macro_rules! Depcrate_matchesmacro_4981 {
() => {
// Module: crate::matches
// Provides: {"macro_4981"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `match`  or `if let` expressions producing a"] # [doc = " `bool` that could be written using `matches!`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability and needless complexity."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This lint falsely triggers, if there are arms with"] # [doc = " `cfg` attributes that remove an arm evaluating to `false`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = Some(5);"] # [doc = ""] # [doc = " let a = match x {"] # [doc = "     Some(0) => true,"] # [doc = "     _ => false,"] # [doc = " };"] # [doc = ""] # [doc = " let a = if let Some(0) = x {"] # [doc = "     true"] # [doc = " } else {"] # [doc = "     false"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = Some(5);"] # [doc = " let a = matches!(x, Some(0));"] # [doc = " ```"] # [clippy :: version = "1.47.0"] pub MATCH_LIKE_MATCHES_MACRO , style , "a match that could be written with the matches! macro" }
};
}
