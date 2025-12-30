// Generated macro for referent_used_exactly_once (function)
macro_rules! Depcrate_needless_borrows_for_generic_argsreferent_used_exactly_once {
() => {
// Module: crate::needless_borrows_for_generic_args
// Provides: {"referent_used_exactly_once"}
// Dependencies: {}
fn referent_used_exactly_once < 'tcx > (cx : & LateContext < 'tcx > , possible_borrowers : & mut Vec < (LocalDefId , PossibleBorrowerMap < 'tcx , 'tcx >) > , reference : & Expr < 'tcx > ,) -> bool { if let Some (mir) = enclosing_mir (cx . tcx , reference . hir_id) && let Some (local) = expr_local (cx . tcx , reference) && let [location] = * local_assignments (mir , local) . as_slice () && let block_data = & mir . basic_blocks [location . block] && let Some (statement) = block_data . statements . get (location . statement_index) && let StatementKind :: Assign (box (_ , Rvalue :: Ref (_ , _ , place))) = statement . kind && ! place . is_indirect_first_projection () { let body_owner_local_def_id = cx . tcx . hir_enclosing_body_owner (reference . hir_id) ; if possible_borrowers . last () . is_none_or (| & (local_def_id , _) | local_def_id != body_owner_local_def_id) { possible_borrowers . push ((body_owner_local_def_id , PossibleBorrowerMap :: new (cx , mir))) ; } let possible_borrower = & mut possible_borrowers . last_mut () . unwrap () . 1 ; possible_borrower . bounded_borrowers (& [local] , & [local , place . local] , place . local , location) && used_exactly_once (mir , place . local) . unwrap_or (false) } else { false } }
};
}
