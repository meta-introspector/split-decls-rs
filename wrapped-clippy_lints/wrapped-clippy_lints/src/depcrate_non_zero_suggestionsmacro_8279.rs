// Generated macro for macro_8279 (macro)
macro_rules! Depcrate_non_zero_suggestionsmacro_8279 {
() => {
// Module: crate::non_zero_suggestions
// Provides: {"macro_8279"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for conversions from `NonZero` types to regular integer types,"] # [doc = " and suggests using `NonZero` types for the target as well."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Converting from `NonZero` types to regular integer types and then back to `NonZero`"] # [doc = " types is less efficient and loses the type-safety guarantees provided by `NonZero` types."] # [doc = " Using `NonZero` types consistently can lead to more optimized code and prevent"] # [doc = " certain classes of errors related to zero values."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::num::{NonZeroU32, NonZeroU64};"] # [doc = ""] # [doc = " fn example(x: u64, y: NonZeroU32) {"] # [doc = "     // Bad: Converting NonZeroU32 to u64 unnecessarily"] # [doc = "     let r1 = x / u64::from(y.get());"] # [doc = "     let r2 = x % u64::from(y.get());"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::num::{NonZeroU32, NonZeroU64};"] # [doc = ""] # [doc = " fn example(x: u64, y: NonZeroU32) {"] # [doc = "     // Good: Preserving the NonZero property"] # [doc = "     let r1 = x / NonZeroU64::from(y);"] # [doc = "     let r2 = x % NonZeroU64::from(y);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.83.0"] pub NON_ZERO_SUGGESTIONS , restriction , "suggests using `NonZero#` from `u#` or `i#` for more efficient and type-safe conversions" }
};
}
