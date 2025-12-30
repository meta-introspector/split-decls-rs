// Generated macro for macro_1479 (macro)
macro_rules! Depcrate_derivable_implsmacro_1479 {
() => {
// Module: crate::derivable_impls
// Provides: {"macro_1479"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects manual `std::default::Default` implementations that are identical to a derived implementation."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is less concise."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Foo {"] # [doc = "     bar: bool"] # [doc = " }"] # [doc = ""] # [doc = " impl Default for Foo {"] # [doc = "     fn default() -> Self {"] # [doc = "         Self {"] # [doc = "             bar: false"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[derive(Default)]"] # [doc = " struct Foo {"] # [doc = "     bar: bool"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Derive macros [sometimes use incorrect bounds](https://github.com/rust-lang/rust/issues/26925)"] # [doc = " in generic types and the user defined `impl` may be more generalized or"] # [doc = " specialized than what derive will produce. This lint can't detect the manual `impl`"] # [doc = " has exactly equal bounds, and therefore this lint is disabled for types with"] # [doc = " generic parameters."] # [clippy :: version = "1.57.0"] pub DERIVABLE_IMPLS , complexity , "manual implementation of the `Default` trait which is equal to a derive" }
};
}
