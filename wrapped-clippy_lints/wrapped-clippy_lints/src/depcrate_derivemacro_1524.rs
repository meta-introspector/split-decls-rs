// Generated macro for macro_1524 (macro)
macro_rules! Depcrate_derivemacro_1524 {
() => {
// Module: crate::derive
// Provides: {"macro_1524"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Lints against manual `PartialEq` implementations for types with a derived `Hash`"] # [doc = " implementation."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The implementation of these traits must agree (for"] # [doc = " example for use with `HashMap`) so it’s probably a bad idea to use a"] # [doc = " default-generated `Hash` implementation with an explicitly defined"] # [doc = " `PartialEq`. In particular, the following must hold for any type:"] # [doc = ""] # [doc = " ```text"] # [doc = " k1 == k2 ⇒ hash(k1) == hash(k2)"] # [doc = " ```"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " #[derive(Hash)]"] # [doc = " struct Foo;"] # [doc = ""] # [doc = " impl PartialEq for Foo {"] # [doc = "     ..."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub DERIVED_HASH_WITH_MANUAL_EQ , correctness , "deriving `Hash` but implementing `PartialEq` explicitly" }
};
}
