// Generated macro for macro_9759 (macro)
macro_rules! Depcrate_suspicious_trait_implmacro_9759 {
() => {
// Module: crate::suspicious_trait_impl
// Provides: {"macro_9759"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Lints for suspicious operations in impls of arithmetic operators, e.g."] # [doc = " subtracting elements in an Add impl."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is probably a typo or copy-and-paste error and not intended."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " impl Add for Foo {"] # [doc = "     type Output = Foo;"] # [doc = ""] # [doc = "     fn add(self, other: Foo) -> Foo {"] # [doc = "         Foo(self.0 - other.0)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SUSPICIOUS_ARITHMETIC_IMPL , suspicious , "suspicious use of operators in impl of arithmetic trait" }
};
}
