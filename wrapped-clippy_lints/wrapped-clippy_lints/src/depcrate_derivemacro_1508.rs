// Generated macro for macro_1508 (macro)
macro_rules! Depcrate_derivemacro_1508 {
() => {
// Module: crate::derive
// Provides: {"macro_1508"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for deriving `serde::Deserialize` on a type that"] # [doc = " has methods using `unsafe`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Deriving `serde::Deserialize` will create a constructor"] # [doc = " that may violate invariants held by another constructor."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " use serde::Deserialize;"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " pub struct Foo {"] # [doc = "     // .."] # [doc = " }"] # [doc = ""] # [doc = " impl Foo {"] # [doc = "     pub fn new() -> Self {"] # [doc = "         // setup here .."] # [doc = "     }"] # [doc = ""] # [doc = "     pub unsafe fn parts() -> (&str, &str) {"] # [doc = "         // assumes invariants hold"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.45.0"] pub UNSAFE_DERIVE_DESERIALIZE , pedantic , "deriving `serde::Deserialize` on a type that has methods using `unsafe`" }
};
}
