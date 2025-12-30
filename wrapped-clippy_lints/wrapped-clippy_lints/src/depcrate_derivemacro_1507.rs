// Generated macro for macro_1507 (macro)
macro_rules! Depcrate_derivemacro_1507 {
() => {
// Module: crate::derive
// Provides: {"macro_1507"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for explicit `Clone` implementations for `Copy`"] # [doc = " types."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " To avoid surprising behavior, these traits should"] # [doc = " agree and the behavior of `Copy` cannot be overridden. In almost all"] # [doc = " situations a `Copy` type should have a `Clone` implementation that does"] # [doc = " nothing more than copy the object, which is what `#[derive(Copy, Clone)]`"] # [doc = " gets you."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " #[derive(Copy)]"] # [doc = " struct Foo;"] # [doc = ""] # [doc = " impl Clone for Foo {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub EXPL_IMPL_CLONE_ON_COPY , pedantic , "implementing `Clone` explicitly on `Copy` types" }
};
}
