// Generated macro for macro_7244 (macro)
macro_rules! Depcrate_methodsmacro_7244 {
() => {
// Module: crate::methods
// Provides: {"macro_7244"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for `NonZero*::new_unchecked()` being used in a `const` context."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Using `NonZero*::new_unchecked()` is an `unsafe` function and requires an `unsafe` context. When used in a"] # [doc = " context evaluated at compilation time, `NonZero*::new().unwrap()` will provide the same result with identical"] # [doc = " runtime performances while not requiring `unsafe`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::num::NonZeroUsize;"] # [doc = " const PLAYERS: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(3) };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::num::NonZeroUsize;"] # [doc = " const PLAYERS: NonZeroUsize = NonZeroUsize::new(3).unwrap();"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub USELESS_NONZERO_NEW_UNCHECKED , complexity , "using `NonZero::new_unchecked()` in a `const` context" }
};
}
