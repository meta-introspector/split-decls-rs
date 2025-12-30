// Generated macro for type_is_inferable_from_arguments (function)
macro_rules! Depcrate_ty_type_certaintytype_is_inferable_from_arguments {
() => {
// Module: crate::ty::type_certainty
// Provides: {"type_is_inferable_from_arguments"}
// Dependencies: {}
# [expect (clippy :: cast_possible_truncation)] fn type_is_inferable_from_arguments (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let Some (callee_def_id) = (match expr . kind { ExprKind :: Call (callee , _) => { let callee_ty = cx . typeck_results () . expr_ty (callee) ; if let ty :: FnDef (callee_def_id , _) = callee_ty . kind () { Some (* callee_def_id) } else { None } } , ExprKind :: MethodCall (_ , _ , _ , _) => cx . typeck_results () . type_dependent_def_id (expr . hir_id) , _ => None , }) else { return false ; } ; let generics = cx . tcx . generics_of (callee_def_id) ; let fn_sig = cx . tcx . fn_sig (callee_def_id) . skip_binder () ; (0 .. (generics . parent_count + generics . own_params . len ()) as u32) . all (| index | { fn_sig . inputs () . iter () . any (| input_ty | contains_param (* input_ty . skip_binder () , index)) }) }
};
}
