macro_rules! deps {
    () => {
        ItemCtxt!();
        SIMDFFIHighlyExperimental!();
    };
}

macro_rules! compute_sig_of_foreign_fn_decl {
    () => {
        deps!();
        fn compute_sig_of_foreign_fn_decl < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , decl : & 'tcx hir :: FnDecl < 'tcx > , abi : ExternAbi , safety : hir :: Safety ,) -> ty :: PolyFnSig < 'tcx > { let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let fty = ItemCtxt :: new (tcx , def_id) . lowerer () . lower_fn_ty (hir_id , safety , abi , decl , None , None) ; if ! tcx . features () . simd_ffi () { let check = | hir_ty : & hir :: Ty < '_ > , ty : Ty < '_ > | { if ty . is_simd () { let snip = tcx . sess . source_map () . span_to_snippet (hir_ty . span) . map_or_else (| _ | String :: new () , | s | format ! (" `{s}`")) ; tcx . dcx () . emit_err (errors :: SIMDFFIHighlyExperimental { span : hir_ty . span , snip }) ; } } ; for (input , ty) in iter :: zip (decl . inputs , fty . inputs () . skip_binder ()) { check (input , * ty) } if let hir :: FnRetTy :: Return (ty) = decl . output { check (ty , fty . output () . skip_binder ()) } } fty }
    };
}

compute_sig_of_foreign_fn_decl!();