// Generated macro for impl_693 (impl)
macro_rules! Depcrate_mirimpl_693 {
() => {
// Module: crate::mir
// Provides: {"impl_693"}
// Dependencies: {}
impl BorrowKind { fn from_hir (m : hir_def :: type_ref :: Mutability) -> Self { match m { hir_def :: type_ref :: Mutability :: Shared => BorrowKind :: Shared , hir_def :: type_ref :: Mutability :: Mut => BorrowKind :: Mut { kind : MutBorrowKind :: Default } , } } fn from_rustc (m : rustc_ast_ir :: Mutability) -> Self { match m { rustc_ast_ir :: Mutability :: Not => BorrowKind :: Shared , rustc_ast_ir :: Mutability :: Mut => BorrowKind :: Mut { kind : MutBorrowKind :: Default } , } } }
};
}
