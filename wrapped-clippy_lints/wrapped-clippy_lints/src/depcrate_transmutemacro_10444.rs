// Generated macro for macro_10444 (macro)
macro_rules! Depcrate_transmutemacro_10444 {
() => {
// Module: crate::transmute
// Provides: {"macro_10444"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for transmutes from `T` to `NonZero<T>`, and suggests the `new_unchecked`"] # [doc = " method instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Transmutes work on any types and thus might cause unsoundness when those types change"] # [doc = " elsewhere. `new_unchecked` only works for the appropriate types instead."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use core::num::NonZero;"] # [doc = " let _: NonZero<u32> = unsafe { std::mem::transmute(123) };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use core::num::NonZero;"] # [doc = " let _: NonZero<u32> = unsafe { NonZero::new_unchecked(123) };"] # [doc = " ```"] # [clippy :: version = "1.69.0"] pub TRANSMUTE_INT_TO_NON_ZERO , complexity , "transmutes from an integer to a non-zero wrapper" }
};
}
