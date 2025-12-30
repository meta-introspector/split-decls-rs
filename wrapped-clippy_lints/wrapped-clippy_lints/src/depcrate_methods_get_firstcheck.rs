// Generated macro for check (function)
macro_rules! Depcrate_methods_get_firstcheck {
() => {
// Module: crate::methods::get_first
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ > , recv : & 'tcx hir :: Expr < '_ > , arg : & 'tcx hir :: Expr < '_ > ,) { if let Some (method_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && let Some (impl_id) = cx . tcx . impl_of_assoc (method_id) && let identity = cx . tcx . type_of (impl_id) . instantiate_identity () && let hir :: ExprKind :: Lit (Spanned { node : LitKind :: Int (Pu128 (0) , _) , .. }) = arg . kind { if identity . is_slice () { let mut app = Applicability :: MachineApplicable ; let slice_name = snippet_with_applicability (cx , recv . span , ".." , & mut app) ; span_lint_and_sugg (cx , GET_FIRST , expr . span , format ! ("accessing first element with `{slice_name}.get(0)`") , "try" , format ! ("{slice_name}.first()") , app ,) ; } else if identity . is_diag_item (cx , sym :: VecDeque) { let mut app = Applicability :: MachineApplicable ; let slice_name = snippet_with_applicability (cx , recv . span , ".." , & mut app) ; span_lint_and_sugg (cx , GET_FIRST , expr . span , format ! ("accessing first element with `{slice_name}.get(0)`") , "try" , format ! ("{slice_name}.front()") , app ,) ; } } }
};
}
