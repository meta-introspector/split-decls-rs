macro_rules! deps {
    () => {
        BorrowckInferCtxt!();
        RegionCtxt!();
        Locations!();
        MirTypeckRegionConstraints!();
        DeferredOpaqueTypeError!();
        UniversalRegions!();
    };
}

macro_rules! apply_computed_concrete_opaque_types {
    () => {
        deps!();
        # [doc = " This function is what actually applies member constraints to the borrowck"] # [doc = " state. It is also responsible to check all uses of the opaques in their"] # [doc = " defining scope."] # [doc = ""] # [doc = " It does this by equating the hidden type of each use with the instantiated final"] # [doc = " hidden type of the opaque."] pub (crate) fn apply_computed_concrete_opaque_types < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , body : & Body < 'tcx > , universal_regions : & UniversalRegions < 'tcx > , region_bound_pairs : & RegionBoundPairs < 'tcx > , known_type_outlives_obligations : & [ty :: PolyTypeOutlivesPredicate < 'tcx >] , constraints : & mut MirTypeckRegionConstraints < 'tcx > , concrete_opaque_types : & mut ConcreteOpaqueTypes < 'tcx > , opaque_types : & [(OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >)] ,) -> Vec < DeferredOpaqueTypeError < 'tcx > > { let tcx = infcx . tcx ; let mut errors = Vec :: new () ; for & (key , hidden_type) in opaque_types { let Some (expected) = get_concrete_opaque_type (concrete_opaque_types , key . def_id) else { if ! tcx . use_typing_mode_borrowck () { if let ty :: Alias (ty :: Opaque , alias_ty) = hidden_type . ty . kind () && alias_ty . def_id == key . def_id . to_def_id () && alias_ty . args == key . args { continue ; } else { unreachable ! ("non-defining use in defining scope") ; } } errors . push (DeferredOpaqueTypeError :: NonDefiningUseInDefiningScope { span : hidden_type . span , opaque_type_key : key , }) ; let guar = tcx . dcx () . span_delayed_bug (hidden_type . span , "non-defining use in the defining scope with no defining uses" ,) ; add_concrete_opaque_type (tcx , concrete_opaque_types , key . def_id , OpaqueHiddenType :: new_error (tcx , guar) ,) ; continue ; } ; let expected = ty :: fold_regions (tcx , expected . instantiate (tcx , key . args) , | re , _dbi | { match re . kind () { ty :: ReErased => infcx . next_nll_region_var (NllRegionVariableOrigin :: Existential { name : None } , | | crate :: RegionCtxt :: Existential (None) ,) , _ => re , } }) ; let locations = Locations :: All (hidden_type . span) ; if let Err (guar) = fully_perform_op_raw (infcx , body , universal_regions , region_bound_pairs , known_type_outlives_obligations , constraints , locations , ConstraintCategory :: OpaqueType , CustomTypeOp :: new (| ocx | { let cause = ObligationCause :: misc (hidden_type . span , body . source . def_id () . expect_local () ,) ; let actual_ty = ocx . normalize (& cause , infcx . param_env , hidden_type . ty) ; let expected_ty = ocx . normalize (& cause , infcx . param_env , expected . ty) ; ocx . eq (& cause , infcx . param_env , actual_ty , expected_ty) . map_err (| _ | NoSolution) } , "equating opaque types" ,) ,) { add_concrete_opaque_type (tcx , concrete_opaque_types , key . def_id , OpaqueHiddenType :: new_error (tcx , guar) ,) ; } } errors }
    };
}

apply_computed_concrete_opaque_types!()