macro_rules! deps {
    () => {
        DefiningUse!();
        RegionCtxt!();
        DeferredOpaqueTypeError!();
        LifetimeMismatchOpaqueParam!();
        ToArgRegionsFolder!();
    };
}

macro_rules! compute_concrete_types_from_defining_uses {
    () => {
        deps!();
        # [instrument (level = "debug" , skip (rcx , concrete_opaque_types , defining_uses , errors))] fn compute_concrete_types_from_defining_uses < 'tcx > (rcx : & RegionCtxt < '_ , 'tcx > , concrete_opaque_types : & mut ConcreteOpaqueTypes < 'tcx > , defining_uses : & [DefiningUse < 'tcx >] , errors : & mut Vec < DeferredOpaqueTypeError < 'tcx > > ,) { let infcx = rcx . infcx ; let tcx = infcx . tcx ; let mut decls_modulo_regions : FxIndexMap < OpaqueTypeKey < 'tcx > , (OpaqueTypeKey < 'tcx > , Span) > = FxIndexMap :: default () ; for & DefiningUse { opaque_type_key , ref arg_regions , hidden_type } in defining_uses { debug ! (? opaque_type_key , ? arg_regions , ? hidden_type) ; let hidden_type = match hidden_type . try_fold_with (& mut ToArgRegionsFolder :: new (rcx , arg_regions)) { Ok (hidden_type) => hidden_type , Err (r) => { debug ! ("UnexpectedHiddenRegion: {:?}" , r) ; errors . push (DeferredOpaqueTypeError :: UnexpectedHiddenRegion { hidden_type , opaque_type_key , member_region : ty :: Region :: new_var (tcx , r) , }) ; let guar = tcx . dcx () . span_delayed_bug (hidden_type . span , "opaque type with non-universal region args" ,) ; ty :: OpaqueHiddenType :: new_error (tcx , guar) } } ; let ty = infcx . infer_opaque_definition_from_instantiation (opaque_type_key , hidden_type) . unwrap_or_else (| _ | { Ty :: new_error_with_message (rcx . infcx . tcx , hidden_type . span , "deferred invalid opaque type args" ,) }) ; if ! rcx . infcx . tcx . use_typing_mode_borrowck () { if let ty :: Alias (ty :: Opaque , alias_ty) = ty . kind () && alias_ty . def_id == opaque_type_key . def_id . to_def_id () && alias_ty . args == opaque_type_key . args { continue ; } } if let Some ((prev_decl_key , prev_span)) = decls_modulo_regions . insert (rcx . infcx . tcx . erase_and_anonymize_regions (opaque_type_key) , (opaque_type_key , hidden_type . span) ,) && let Some ((arg1 , arg2)) = std :: iter :: zip (prev_decl_key . iter_captured_args (infcx . tcx) . map (| (_ , arg) | arg) , opaque_type_key . iter_captured_args (infcx . tcx) . map (| (_ , arg) | arg) ,) . find (| (arg1 , arg2) | arg1 != arg2) { errors . push (DeferredOpaqueTypeError :: LifetimeMismatchOpaqueParam (LifetimeMismatchOpaqueParam { arg : arg1 , prev : arg2 , span : prev_span , prev_span : hidden_type . span , } ,)) ; } add_concrete_opaque_type (tcx , concrete_opaque_types , opaque_type_key . def_id , OpaqueHiddenType { span : hidden_type . span , ty } ,) ; } }
    };
}

compute_concrete_types_from_defining_uses!();