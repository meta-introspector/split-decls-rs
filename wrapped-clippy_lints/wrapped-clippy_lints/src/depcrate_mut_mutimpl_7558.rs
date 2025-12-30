// Generated macro for impl_7558 (impl)
macro_rules! Depcrate_mut_mutimpl_7558 {
() => {
// Module: crate::mut_mut
// Provides: {"impl_7558"}
// Dependencies: {}
impl < 'tcx > intravisit :: Visitor < 'tcx > for MutVisitor < '_ , 'tcx > { fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < '_ >) { if expr . span . in_external_macro (self . cx . sess () . source_map ()) { return ; } if let Some (higher :: ForLoop { arg , body , .. }) = higher :: ForLoop :: hir (expr) { intravisit :: walk_expr (self , arg) ; intravisit :: walk_expr (self , body) ; } else if let hir :: ExprKind :: AddrOf (hir :: BorrowKind :: Ref , hir :: Mutability :: Mut , e) = expr . kind { if let hir :: ExprKind :: AddrOf (hir :: BorrowKind :: Ref , hir :: Mutability :: Mut , _) = e . kind { span_lint_hir (self . cx , MUT_MUT , expr . hir_id , expr . span , "generally you want to avoid `&mut &mut _` if possible" ,) ; } else if let ty :: Ref (_ , ty , hir :: Mutability :: Mut) = self . cx . typeck_results () . expr_ty (e) . kind () && ty . peel_refs () . is_sized (self . cx . tcx , self . cx . typing_env ()) { span_lint_hir (self . cx , MUT_MUT , expr . hir_id , expr . span , "this expression mutably borrows a mutable reference. Consider reborrowing" ,) ; } } } }
};
}
