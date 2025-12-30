// Generated macro for macro_7978 (macro)
macro_rules! Depcrate_non_canonical_implsmacro_7978 {
() => {
// Module: crate::non_canonical_impls
// Provides: {"macro_7978"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for non-canonical implementations of `Clone` when `Copy` is already implemented."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If both `Clone` and `Copy` are implemented, they must agree. This can done by dereferencing"] # [doc = " `self` in `Clone`'s implementation, which will avoid any possibility of the implementations"] # [doc = " becoming out of sync."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " #[derive(Eq, PartialEq)]"] # [doc = " struct A(u32);"] # [doc = ""] # [doc = " impl Clone for A {"] # [doc = "     fn clone(&self) -> Self {"] # [doc = "         Self(self.0)"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " impl Copy for A {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " #[derive(Eq, PartialEq)]"] # [doc = " struct A(u32);"] # [doc = ""] # [doc = " impl Clone for A {"] # [doc = "     fn clone(&self) -> Self {"] # [doc = "         *self"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " impl Copy for A {}"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub NON_CANONICAL_CLONE_IMPL , suspicious , "non-canonical implementation of `Clone` on a `Copy` type" }
};
}
