// Generated macro for macro_4284 (macro)
macro_rules! Depcrate_manual_non_exhaustivemacro_4284 {
() => {
// Module: crate::manual_non_exhaustive
// Provides: {"macro_4284"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manual implementations of the non-exhaustive pattern."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using the #[non_exhaustive] attribute expresses better the intent"] # [doc = " and allows possible optimizations when applied to enums."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct S {"] # [doc = "     pub a: i32,"] # [doc = "     pub b: i32,"] # [doc = "     _c: (),"] # [doc = " }"] # [doc = ""] # [doc = " enum E {"] # [doc = "     A,"] # [doc = "     B,"] # [doc = "     #[doc(hidden)]"] # [doc = "     _C,"] # [doc = " }"] # [doc = ""] # [doc = " struct T(pub i32, pub i32, ());"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[non_exhaustive]"] # [doc = " struct S {"] # [doc = "     pub a: i32,"] # [doc = "     pub b: i32,"] # [doc = " }"] # [doc = ""] # [doc = " #[non_exhaustive]"] # [doc = " enum E {"] # [doc = "     A,"] # [doc = "     B,"] # [doc = " }"] # [doc = ""] # [doc = " #[non_exhaustive]"] # [doc = " struct T(pub i32, pub i32);"] # [doc = " ```"] # [clippy :: version = "1.45.0"] pub MANUAL_NON_EXHAUSTIVE , style , "manual implementations of the non-exhaustive pattern can be simplified using #[non_exhaustive]" }
};
}
