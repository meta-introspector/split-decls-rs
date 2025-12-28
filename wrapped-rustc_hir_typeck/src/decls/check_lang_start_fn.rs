macro_rules! check_lang_start_fn {
    () => {
        fn check_lang_start_fn < 'tcx > (tcx : TyCtxt < 'tcx > , fn_sig : ty :: FnSig < 'tcx > , def_id : LocalDefId) { let generics = tcx . generics_of (def_id) ; let fn_generic = generics . param_at (0 , tcx) ; let generic_ty = Ty :: new_param (tcx , fn_generic . index , fn_generic . name) ; let main_fn_ty = Ty :: new_fn_ptr (tcx , Binder :: dummy (tcx . mk_fn_sig ([] , generic_ty , false , hir :: Safety :: Safe , ExternAbi :: Rust)) ,) ; let expected_sig = ty :: Binder :: dummy (tcx . mk_fn_sig ([main_fn_ty , tcx . types . isize , Ty :: new_imm_ptr (tcx , Ty :: new_imm_ptr (tcx , tcx . types . u8)) , tcx . types . u8 ,] , tcx . types . isize , false , fn_sig . safety , ExternAbi :: Rust ,)) ; let _ = check_function_signature (tcx , ObligationCause :: new (tcx . def_span (def_id) , def_id , ObligationCauseCode :: LangFunctionType (sym :: start) ,) , def_id . into () , expected_sig ,) ; }
    };
}

check_lang_start_fn!();