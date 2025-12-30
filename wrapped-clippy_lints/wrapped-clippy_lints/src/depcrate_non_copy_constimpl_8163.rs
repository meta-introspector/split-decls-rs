// Generated macro for impl_8163 (impl)
macro_rules! Depcrate_non_copy_constimpl_8163 {
() => {
// Module: crate::non_copy_const
// Provides: {"impl_8163"}
// Dependencies: {}
impl < 'tcx > BorrowSource < 'tcx > { fn new (tcx : TyCtxt < 'tcx > , expr : & 'tcx Expr < 'tcx > , cause : BorrowCause) -> Self { let (expr , cause) = if matches ! (cause , BorrowCause :: AutoBorrow) && let Node :: Expr (parent) = tcx . parent_hir_node (expr . hir_id) { match parent . kind { ExprKind :: Unary (UnOp :: Deref , _) => (parent , BorrowCause :: Deref) , ExprKind :: Index (..) => (parent , BorrowCause :: Index) , ExprKind :: Field (..) => (parent , BorrowCause :: AutoDerefField) , _ => (expr , cause) , } } else { (expr , cause) } ; Self { expr , cause } } }
};
}
