macro_rules! deps {
    () => {
        HirPlaceholderCollector!();
        ItemCtxt!();
    };
}

macro_rules! recover_infer_ret_ty {
    () => {
        deps!();
        fn recover_infer_ret_ty < 'tcx > (icx : & ItemCtxt < 'tcx > , infer_ret_ty : & 'tcx hir :: Ty < 'tcx > , generics : & 'tcx hir :: Generics < 'tcx > , def_id : LocalDefId ,) -> ty :: PolyFnSig < 'tcx > { let tcx = icx . tcx ; let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let fn_sig = tcx . typeck (def_id) . liberated_fn_sigs () [hir_id] ; let has_region_params = generics . params . iter () . any (| param | match param . kind { GenericParamKind :: Lifetime { .. } => true , _ => false , }) ; let fn_sig = fold_regions (tcx , fn_sig , | r , _ | match r . kind () { ty :: ReErased => { if has_region_params { ty :: Region :: new_error_with_message (tcx , DUMMY_SP , "erased region is not allowed here in return type" ,) } else { tcx . lifetimes . re_static } } _ => r , }) ; let mut visitor = HirPlaceholderCollector :: default () ; visitor . visit_ty_unambig (infer_ret_ty) ; let mut diag = bad_placeholder (icx . lowerer () , visitor . spans , "return type") ; let ret_ty = fn_sig . output () ; let mut recovered_ret_ty = None ; if let Some (suggestable_ret_ty) = ret_ty . make_suggestable (tcx , false , None) { diag . span_suggestion (infer_ret_ty . span , "replace with the correct return type" , suggestable_ret_ty , Applicability :: MachineApplicable ,) ; recovered_ret_ty = Some (suggestable_ret_ty) ; } else if let Some (sugg) = suggest_impl_trait (& tcx . infer_ctxt () . build (TypingMode :: non_body_analysis ()) , tcx . param_env (def_id) , ret_ty ,) { diag . span_suggestion (infer_ret_ty . span , "replace with an appropriate return type" , sugg , Applicability :: MachineApplicable ,) ; } else if ret_ty . is_closure () { diag . help ("consider using an `Fn`, `FnMut`, or `FnOnce` trait bound") ; } if ret_ty . is_closure () { diag . note ("for more information on `Fn` traits and closure types, see \
                     https://doc.rust-lang.org/book/ch13-01-closures.html" ,) ; } let guar = diag . emit () ; ty :: Binder :: dummy (tcx . mk_fn_sig (fn_sig . inputs () . iter () . copied () , recovered_ret_ty . unwrap_or_else (| | Ty :: new_error (tcx , guar)) , fn_sig . c_variadic , fn_sig . safety , fn_sig . abi ,)) }
    };
}

recover_infer_ret_ty!()