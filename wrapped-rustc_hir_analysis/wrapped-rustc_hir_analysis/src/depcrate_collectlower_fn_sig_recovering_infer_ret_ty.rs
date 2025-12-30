// Generated macro for lower_fn_sig_recovering_infer_ret_ty (function)
macro_rules! Depcrate_collectlower_fn_sig_recovering_infer_ret_ty {
() => {
// Module: crate::collect
// Provides: {"lower_fn_sig_recovering_infer_ret_ty"}
// Dependencies: {}
fn lower_fn_sig_recovering_infer_ret_ty < 'tcx > (icx : & ItemCtxt < 'tcx > , sig : & 'tcx hir :: FnSig < 'tcx > , generics : & 'tcx hir :: Generics < 'tcx > , def_id : LocalDefId ,) -> ty :: PolyFnSig < 'tcx > { if let Some (infer_ret_ty) = sig . decl . output . is_suggestable_infer_ty () { return recover_infer_ret_ty (icx , infer_ret_ty , generics , def_id) ; } icx . lowerer () . lower_fn_ty (icx . tcx () . local_def_id_to_hir_id (def_id) , sig . header . safety () , sig . header . abi , sig . decl , Some (generics) , None ,) }
};
}
