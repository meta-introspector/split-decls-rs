// Generated macro for macro_2148 (macro)
macro_rules! Depcrate_fallible_impl_frommacro_2148 {
() => {
// Module: crate::fallible_impl_from
// Provides: {"macro_2148"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for impls of `From<..>` that contain `panic!()` or `unwrap()`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `TryFrom` should be used if there's a possibility of failure."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Foo(i32);"] # [doc = ""] # [doc = " impl From<String> for Foo {"] # [doc = "     fn from(s: String) -> Self {"] # [doc = "         Foo(s.parse().unwrap())"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct Foo(i32);"] # [doc = ""] # [doc = " impl TryFrom<String> for Foo {"] # [doc = "     type Error = ();"] # [doc = "     fn try_from(s: String) -> Result<Self, Self::Error> {"] # [doc = "         if let Ok(parsed) = s.parse() {"] # [doc = "             Ok(Foo(parsed))"] # [doc = "         } else {"] # [doc = "             Err(())"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub FALLIBLE_IMPL_FROM , nursery , "Warn on impls of `From<..>` that contain `panic!()` or `unwrap()`" }
};
}
