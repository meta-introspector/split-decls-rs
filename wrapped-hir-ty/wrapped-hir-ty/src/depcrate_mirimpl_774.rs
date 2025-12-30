// Generated macro for impl_774 (impl)
macro_rules! Depcrate_mirimpl_774 {
() => {
// Module: crate::mir
// Provides: {"impl_774"}
// Dependencies: {}
impl BorrowKind { fn from_hir (m : hir_def :: type_ref :: Mutability) -> Self { match m { hir_def :: type_ref :: Mutability :: Shared => BorrowKind :: Shared , hir_def :: type_ref :: Mutability :: Mut => BorrowKind :: Mut { kind : MutBorrowKind :: Default } , } } fn from_chalk (m : Mutability) -> Self { match m { Mutability :: Not => BorrowKind :: Shared , Mutability :: Mut => BorrowKind :: Mut { kind : MutBorrowKind :: Default } , } } }
};
}
