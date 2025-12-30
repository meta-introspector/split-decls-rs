// Generated macro for impl_381 (impl)
macro_rules! Depcrateimpl_381 {
() => {
// Module: crate
// Provides: {"impl_381"}
// Dependencies: {}
impl From < hir_ty :: next_solver :: Mutability > for Access { fn from (mutability : hir_ty :: next_solver :: Mutability) -> Access { match mutability { hir_ty :: next_solver :: Mutability :: Not => Access :: Shared , hir_ty :: next_solver :: Mutability :: Mut => Access :: Exclusive , } } }
};
}
