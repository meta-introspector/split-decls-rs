macro_rules! deps {
    () => {
        ItemCtxt!();
    };
}

macro_rules! lower_fn_sig_recovering_infer_ret_ty {
    () => {
        deps!();
        fn lower_fn_sig_recovering_infer_ret_ty < 'tcx > (icx : & ItemCtxt < 'tcx > , sig : & 'tcx hir :: FnSig < 'tcx > , generics : & 'tcx hir :: Generics < 'tcx > , def_id : LocalDefId ,) -> ty :: PolyFnSig < 'tcx > { if let Some (infer_ret_ty) = sig . decl . output . is_suggestable_infer_ty () { return recover_infer_ret_ty (icx , infer_ret_ty , generics , def_id) ; } icx . lowerer () . lower_fn_ty (icx . tcx () . local_def_id_to_hir_id (def_id) , sig . header . safety () , sig . header . abi , sig . decl , Some (generics) , None ,) }
    };
}

lower_fn_sig_recovering_infer_ret_ty!();