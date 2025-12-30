// Generated macro for macro_7008 (macro)
macro_rules! Depcrate_methodsmacro_7008 {
() => {
// Module: crate::methods
// Provides: {"macro_7008"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `.checked_add/sub(x).unwrap_or(MAX/MIN)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " These can be written simply with `saturating_add/sub` methods."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let y: u32 = 0;"] # [doc = " # let x: u32 = 100;"] # [doc = " let add = x.checked_add(y).unwrap_or(u32::MAX);"] # [doc = " let sub = x.checked_sub(y).unwrap_or(u32::MIN);"] # [doc = " ```"] # [doc = ""] # [doc = " can be written using dedicated methods for saturating addition/subtraction as:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let y: u32 = 0;"] # [doc = " # let x: u32 = 100;"] # [doc = " let add = x.saturating_add(y);"] # [doc = " let sub = x.saturating_sub(y);"] # [doc = " ```"] # [clippy :: version = "1.39.0"] pub MANUAL_SATURATING_ARITHMETIC , style , "`.checked_add/sub(x).unwrap_or(MAX/MIN)`" }
};
}
