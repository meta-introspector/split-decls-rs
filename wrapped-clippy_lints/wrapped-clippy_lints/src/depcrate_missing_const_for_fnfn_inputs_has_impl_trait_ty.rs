// Generated macro for fn_inputs_has_impl_trait_ty (function)
macro_rules! Depcrate_missing_const_for_fnfn_inputs_has_impl_trait_ty {
() => {
// Module: crate::missing_const_for_fn
// Provides: {"fn_inputs_has_impl_trait_ty"}
// Dependencies: {}
# [doc = " Return `true` when the given `def_id` is a function that has `impl Trait` ty as one of"] # [doc = " its parameter types."] fn fn_inputs_has_impl_trait_ty (cx : & LateContext < '_ > , def_id : LocalDefId) -> bool { let inputs = cx . tcx . fn_sig (def_id) . instantiate_identity () . inputs () . skip_binder () ; inputs . iter () . any (| input | { matches ! (input . kind () , ty :: Alias (ty :: AliasTyKind :: Free , alias_ty) if cx . tcx . type_of (alias_ty . def_id) . skip_binder () . is_impl_trait ()) }) }
};
}
