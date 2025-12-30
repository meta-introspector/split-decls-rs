// Generated macro for macro_8867 (macro)
macro_rules! Depcrate_partialeq_ne_implmacro_8867 {
() => {
// Module: crate::partialeq_ne_impl
// Provides: {"macro_8867"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manual re-implementations of `PartialEq::ne`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `PartialEq::ne` is required to always return the"] # [doc = " negated result of `PartialEq::eq`, which is exactly what the default"] # [doc = " implementation does. Therefore, there should never be any need to"] # [doc = " re-implement it."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Foo;"] # [doc = ""] # [doc = " impl PartialEq for Foo {"] # [doc = "     fn eq(&self, other: &Foo) -> bool { true }"] # [doc = "     fn ne(&self, other: &Foo) -> bool { !(self == other) }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub PARTIALEQ_NE_IMPL , complexity , "re-implementing `PartialEq::ne`" }
};
}
