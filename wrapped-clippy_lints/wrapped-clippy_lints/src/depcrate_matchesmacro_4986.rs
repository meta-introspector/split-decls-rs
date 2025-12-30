// Generated macro for macro_4986 (macro)
macro_rules! Depcrate_matchesmacro_4986 {
() => {
// Module: crate::matches
// Provides: {"macro_4986"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks if a `match` or `if let` expression can be simplified using"] # [doc = " `.unwrap_or_default()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It can be done in one call with `.unwrap_or_default()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x: Option<String> = Some(String::new());"] # [doc = " let y: String = match x {"] # [doc = "     Some(v) => v,"] # [doc = "     None => String::new(),"] # [doc = " };"] # [doc = ""] # [doc = " let x: Option<Vec<String>> = Some(Vec::new());"] # [doc = " let y: Vec<String> = if let Some(v) = x {"] # [doc = "     v"] # [doc = " } else {"] # [doc = "     Vec::new()"] # [doc = " };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x: Option<String> = Some(String::new());"] # [doc = " let y: String = x.unwrap_or_default();"] # [doc = ""] # [doc = " let x: Option<Vec<String>> = Some(Vec::new());"] # [doc = " let y: Vec<String> = x.unwrap_or_default();"] # [doc = " ```"] # [clippy :: version = "1.79.0"] pub MANUAL_UNWRAP_OR_DEFAULT , suspicious , "check if a `match` or `if let` can be simplified with `unwrap_or_default`" }
};
}
