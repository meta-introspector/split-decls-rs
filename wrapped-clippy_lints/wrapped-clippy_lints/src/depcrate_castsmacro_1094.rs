// Generated macro for macro_1094 (macro)
macro_rules! Depcrate_castsmacro_1094 {
() => {
// Module: crate::casts
// Provides: {"macro_1094"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the usage of `as _` conversion using inferred type."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " The conversion might include lossy conversion or a dangerous cast that might go"] # [doc = " undetected due to the type being inferred."] # [doc = ""] # [doc = " The lint is allowed by default as using `_` is less wordy than always specifying the type."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(n: usize) {}"] # [doc = " let n: u16 = 256;"] # [doc = " foo(n as _);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo(n: usize) {}"] # [doc = " let n: u16 = 256;"] # [doc = " foo(n as usize);"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub AS_UNDERSCORE , restriction , "detects `as _` conversion" }
};
}
