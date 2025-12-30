// Generated macro for check_borrow_predicate (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedcheck_borrow_predicate {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"check_borrow_predicate"}
// Dependencies: {}
fn check_borrow_predicate < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { if let ExprKind :: MethodCall (_ , caller , & [arg] , _) = expr . kind && let Some (method_def_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && cx . tcx . trait_of_assoc (method_def_id) . is_none () && let Some (borrow_id) = cx . tcx . get_diagnostic_item (sym :: Borrow) && cx . tcx . predicates_of (method_def_id) . predicates . iter () . any (| (pred , _) | { if let ClauseKind :: Trait (trait_pred) = pred . kind () . skip_binder () && trait_pred . polarity == ty :: PredicatePolarity :: Positive && trait_pred . trait_ref . def_id == borrow_id { true } else { false } }) && let caller_ty = cx . typeck_results () . expr_ty (caller) && let Some (key_ty) = std_map_key (cx , caller_ty) && ! key_ty . is_ref () { check_if_applicable_to_argument (cx , & arg) ; } }
};
}
