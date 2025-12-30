// Generated macro for macro_7174 (macro)
macro_rules! Depcrate_methodsmacro_7174 {
() => {
// Module: crate::methods
// Provides: {"macro_7174"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `str::splitn(2, _)`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `split_once` is both clearer in intent and slightly more efficient."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let s = \"key=value=add\";"] # [doc = " let (key, value) = s.splitn(2, '=').next_tuple()?;"] # [doc = " let value = s.splitn(2, '=').nth(1)?;"] # [doc = ""] # [doc = " let mut parts = s.splitn(2, '=');"] # [doc = " let key = parts.next()?;"] # [doc = " let value = parts.next()?;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " let s = \"key=value=add\";"] # [doc = " let (key, value) = s.split_once('=')?;"] # [doc = " let value = s.split_once('=')?.1;"] # [doc = ""] # [doc = " let (key, value) = s.split_once('=')?;"] # [doc = " ```"] # [doc = ""] # [doc = " ### Limitations"] # [doc = " The multiple statement variant currently only detects `iter.next()?`/`iter.next().unwrap()`"] # [doc = " in two separate `let` statements that immediately follow the `splitn()`"] # [clippy :: version = "1.57.0"] pub MANUAL_SPLIT_ONCE , complexity , "replace `.splitn(2, pat)` with `.split_once(pat)`" }
};
}
