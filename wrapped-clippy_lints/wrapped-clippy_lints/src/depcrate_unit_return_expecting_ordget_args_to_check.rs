// Generated macro for get_args_to_check (function)
macro_rules! Depcrate_unit_return_expecting_ordget_args_to_check {
() => {
// Module: crate::unit_return_expecting_ord
// Provides: {"get_args_to_check"}
// Dependencies: {}
fn get_args_to_check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , args_len : usize , fn_mut_trait : DefId , ord_trait : Option < DefId > , partial_ord_trait : Option < DefId > ,) -> Vec < (usize , Symbol) > { let mut args_to_check = Vec :: new () ; if let Some (def_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) { let fn_sig = cx . tcx . fn_sig (def_id) . instantiate_identity () ; let generics = cx . tcx . predicates_of (def_id) ; let [fn_mut_preds , ord_preds , partial_ord_preds] = get_trait_predicates_for_trait_ids (cx , generics , & [Some (fn_mut_trait) , ord_trait , partial_ord_trait]) ; if fn_mut_preds . is_empty () { return vec ! [] ; } let inputs_output = cx . tcx . instantiate_bound_regions_with_erased (fn_sig . inputs_and_output ()) ; inputs_output . iter () . rev () . skip (1) . rev () . enumerate () . for_each (| (i , inp) | { for trait_pred in & fn_mut_preds { if trait_pred . self_ty () == inp && let Some (return_ty_pred) = get_projection_pred (cx , generics , * trait_pred) { if ord_preds . iter () . any (| ord | Some (ord . self_ty ()) == return_ty_pred . term . as_type ()) { args_to_check . push ((i , sym :: Ord)) ; if args_to_check . len () == args_len - 1 { break ; } } else if partial_ord_preds . iter () . any (| pord | pord . self_ty () == return_ty_pred . term . expect_type ()) { args_to_check . push ((i , sym :: PartialOrd)) ; if args_to_check . len () == args_len - 1 { break ; } } } } }) ; } args_to_check }
};
}
