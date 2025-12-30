// Generated macro for macro_7075 (macro)
macro_rules! Depcrate_methodsmacro_7075 {
() => {
// Module: crate::methods
// Provides: {"macro_7075"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `TryInto::try_into` and `TryFrom::try_from` when their infallible counterparts"] # [doc = " could be used."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " In those cases, the `TryInto` and `TryFrom` trait implementation is a blanket impl that forwards"] # [doc = " to `Into` or `From`, which always succeeds."] # [doc = " The returned `Result<_, Infallible>` requires error handling to get the contained value"] # [doc = " even though the conversion can never fail."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " let _: Result<i64, _> = 1i32.try_into();"] # [doc = " let _: Result<i64, _> = <_>::try_from(1i32);"] # [doc = " ```"] # [doc = " Use `from`/`into` instead:"] # [doc = " ```rust"] # [doc = " let _: i64 = 1i32.into();"] # [doc = " let _: i64 = <_>::from(1i32);"] # [doc = " ```"] # [clippy :: version = "1.75.0"] pub UNNECESSARY_FALLIBLE_CONVERSIONS , style , "calling the `try_from` and `try_into` trait methods when `From`/`Into` is implemented" }
};
}
