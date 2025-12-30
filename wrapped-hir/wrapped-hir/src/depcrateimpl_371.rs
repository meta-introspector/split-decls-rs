// Generated macro for impl_371 (impl)
macro_rules! Depcrateimpl_371 {
() => {
// Module: crate
// Provides: {"impl_371"}
// Dependencies: {}
impl From < hir_ty :: Mutability > for Access { fn from (mutability : hir_ty :: Mutability) -> Access { match mutability { hir_ty :: Mutability :: Not => Access :: Shared , hir_ty :: Mutability :: Mut => Access :: Exclusive , } } }
};
}
