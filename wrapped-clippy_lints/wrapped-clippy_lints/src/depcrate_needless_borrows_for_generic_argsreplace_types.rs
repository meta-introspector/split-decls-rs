// Generated macro for replace_types (function)
macro_rules! Depcrate_needless_borrows_for_generic_argsreplace_types {
() => {
// Module: crate::needless_borrows_for_generic_args
// Provides: {"replace_types"}
// Dependencies: {}
fn replace_types < 'tcx > (cx : & LateContext < 'tcx > , param_ty : ParamTy , new_ty : Ty < 'tcx > , fn_sig : FnSig < 'tcx > , arg_index : usize , projection_predicates : & [ProjectionPredicate < 'tcx >] , args : & mut [GenericArg < 'tcx >] ,) -> bool { let mut replaced = DenseBitSet :: new_empty (args . len ()) ; let mut deque = VecDeque :: with_capacity (args . len ()) ; deque . push_back ((param_ty , new_ty)) ; while let Some ((param_ty , new_ty)) = deque . pop_front () { if ! fn_sig . inputs_and_output . iter () . enumerate () . all (| (i , ty) | (replaced . is_empty () && i == arg_index) || ! ty . contains (param_ty . to_ty (cx . tcx))) { return false ; } args [param_ty . index as usize] = GenericArg :: from (new_ty) ; if replaced . insert (param_ty . index) { for projection_predicate in projection_predicates { if projection_predicate . projection_term . self_ty () == param_ty . to_ty (cx . tcx) && let Some (term_ty) = projection_predicate . term . as_type () && let ty :: Param (term_param_ty) = term_ty . kind () { let projection = projection_predicate . projection_term . with_replaced_self_ty (cx . tcx , new_ty) . expect_ty (cx . tcx) . to_ty (cx . tcx) ; if let Ok (projected_ty) = cx . tcx . try_normalize_erasing_regions (cx . typing_env () , projection) && args [term_param_ty . index as usize] != GenericArg :: from (projected_ty) { deque . push_back ((* term_param_ty , projected_ty)) ; } } } } } true }
};
}
