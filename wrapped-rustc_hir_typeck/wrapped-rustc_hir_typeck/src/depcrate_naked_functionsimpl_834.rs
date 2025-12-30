// Generated macro for impl_834 (impl)
macro_rules! Depcrate_naked_functionsimpl_834 {
() => {
// Module: crate::naked_functions
// Provides: {"impl_834"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for CheckParameters < 'tcx > { fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Path (hir :: QPath :: Resolved (_ , hir :: Path { res : hir :: def :: Res :: Local (var_hir_id) , .. } ,)) = expr . kind { if self . params . contains (var_hir_id) { self . tcx . dcx () . emit_err (ParamsNotAllowed { span : expr . span }) ; return ; } } hir :: intravisit :: walk_expr (self , expr) ; } }
};
}
