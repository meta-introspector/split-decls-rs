// Generated macro for macro_10056 (macro)
macro_rules! Depcrate_suspicious_trait_implmacro_10056 {
() => {
// Module: crate::suspicious_trait_impl
// Provides: {"macro_10056"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Lints for suspicious operations in impls of OpAssign, e.g."] # [doc = " subtracting elements in an AddAssign impl."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is probably a typo or copy-and-paste error and not intended."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " impl AddAssign for Foo {"] # [doc = "     fn add_assign(&mut self, other: Foo) {"] # [doc = "         *self = *self - other;"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SUSPICIOUS_OP_ASSIGN_IMPL , suspicious , "suspicious use of operators in impl of OpAssign trait" }
};
}
