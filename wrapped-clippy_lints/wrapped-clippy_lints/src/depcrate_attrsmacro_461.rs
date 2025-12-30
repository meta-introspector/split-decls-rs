// Generated macro for macro_461 (macro)
macro_rules! Depcrate_attrsmacro_461 {
() => {
// Module: crate::attrs
// Provides: {"macro_461"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for items that have the same kind of attributes with mixed styles (inner/outer)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Having both style of said attributes makes it more complicated to read code."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This lint currently has false-negatives when mixing same attributes"] # [doc = " but they have different path symbols, for example:"] # [doc = " ```ignore"] # [doc = " #[custom_attribute]"] # [doc = " pub fn foo() {"] # [doc = "     #![my_crate::custom_attribute]"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[cfg(linux)]"] # [doc = " pub fn foo() {"] # [doc = "     #![cfg(windows)]"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[cfg(linux)]"] # [doc = " #[cfg(windows)]"] # [doc = " pub fn foo() {"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub MIXED_ATTRIBUTES_STYLE , style , "item has both inner and outer attributes" }
};
}
