// Generated macro for macro_10090 (macro)
macro_rules! Depcrate_swapmacro_10090 {
() => {
// Module: crate::swap
// Provides: {"macro_10090"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manual swapping."] # [doc = ""] # [doc = " Note that the lint will not be emitted in const blocks, as the suggestion would not be applicable."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `std::mem::swap` function exposes the intent better"] # [doc = " without deinitializing or copying either variable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut a = 42;"] # [doc = " let mut b = 1337;"] # [doc = ""] # [doc = " let t = b;"] # [doc = " b = a;"] # [doc = " a = t;"] # [doc = " ```"] # [doc = " Use std::mem::swap():"] # [doc = " ```no_run"] # [doc = " let mut a = 1;"] # [doc = " let mut b = 2;"] # [doc = " std::mem::swap(&mut a, &mut b);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MANUAL_SWAP , complexity , "manual swap of two variables" }
};
}
