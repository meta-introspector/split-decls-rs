// Generated macro for macro_4973 (macro)
macro_rules! Depcrate_matchesmacro_4973 {
() => {
// Module: crate::matches
// Provides: {"macro_4973"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for match which is used to add a reference to an"] # [doc = " `Option` value."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `as_ref()` or `as_mut()` instead is shorter."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x: Option<()> = None;"] # [doc = ""] # [doc = " let r: Option<&()> = match x {"] # [doc = "     None => None,"] # [doc = "     Some(ref v) => Some(v),"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x: Option<()> = None;"] # [doc = ""] # [doc = " let r: Option<&()> = x.as_ref();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MATCH_AS_REF , complexity , "a `match` on an Option value instead of using `as_ref()` or `as_mut`" }
};
}
