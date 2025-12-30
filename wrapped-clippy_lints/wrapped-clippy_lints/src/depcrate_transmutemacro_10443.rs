// Generated macro for macro_10443 (macro)
macro_rules! Depcrate_transmutemacro_10443 {
() => {
// Module: crate::transmute
// Provides: {"macro_10443"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for transmutes from an integer to a `bool`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This might result in an invalid in-memory representation of a `bool`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = 1_u8;"] # [doc = " unsafe {"] # [doc = "     let _: bool = std::mem::transmute(x); // where x: u8"] # [doc = " }"] # [doc = ""] # [doc = " // should be:"] # [doc = " let _: bool = x != 0;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub TRANSMUTE_INT_TO_BOOL , complexity , "transmutes from an integer to a `bool`" }
};
}
