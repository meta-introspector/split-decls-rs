// Generated macro for impl_10578 (impl)
macro_rules! Depcrate_unnecessary_semicolonimpl_10578 {
() => {
// Module: crate::unnecessary_semicolon
// Provides: {"impl_10578"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnnecessarySemicolon { fn check_block (& mut self , cx : & LateContext < '_ > , block : & Block < '_ >) { self . handle_block (cx , block , true) ; } fn check_block_post (& mut self , cx : & LateContext < '_ > , block : & Block < '_ >) { self . handle_block (cx , block , false) ; } fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & Stmt < 'tcx >) { if let StmtKind :: Semi (expr) = stmt . kind && ! stmt . span . from_expansion () && ! expr . span . from_expansion () && matches ! (expr . kind , ExprKind :: If (..) | ExprKind :: Match (_ , _ , MatchSource :: Normal | MatchSource :: Postfix)) && cx . typeck_results () . expr_ty (expr) . is_unit () && cx . tcx . hir_attrs (stmt . hir_id) . is_empty () { if let Some (block_is_unit) = self . is_last_in_block (stmt) { if cx . tcx . sess . edition () <= Edition2021 && leaks_droppable_temporary_with_limited_lifetime (cx , expr) { return ; } if ! block_is_unit { return ; } } let semi_span = expr . span . shrink_to_hi () . to (stmt . span . shrink_to_hi ()) ; span_lint_and_sugg (cx , UNNECESSARY_SEMICOLON , semi_span , "unnecessary semicolon" , "remove" , String :: new () , Applicability :: MachineApplicable ,) ; } } }
};
}
