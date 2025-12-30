// Generated macro for is_mixed_projection_predicate (function)
macro_rules! Depcrate_needless_borrows_for_generic_argsis_mixed_projection_predicate {
() => {
// Module: crate::needless_borrows_for_generic_args
// Provides: {"is_mixed_projection_predicate"}
// Dependencies: {}
fn is_mixed_projection_predicate < 'tcx > (cx : & LateContext < 'tcx > , callee_def_id : DefId , projection_predicate : & ProjectionPredicate < 'tcx > ,) -> bool { let generics = cx . tcx . generics_of (callee_def_id) ; if let Some (term_ty) = projection_predicate . term . as_type () && let ty :: Param (term_param_ty) = term_ty . kind () && (term_param_ty . index as usize) < generics . parent_count { let mut projection_term = projection_predicate . projection_term ; loop { match * projection_term . self_ty () . kind () { ty :: Alias (ty :: Projection , inner_projection_ty) => { projection_term = inner_projection_ty . into () ; } , ty :: Param (param_ty) => { return (param_ty . index as usize) >= generics . parent_count ; } , _ => { return false ; } , } } } else { false } }
};
}
