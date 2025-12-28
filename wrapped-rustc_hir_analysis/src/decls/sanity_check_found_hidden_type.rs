macro_rules! sanity_check_found_hidden_type {
    () => {
        fn sanity_check_found_hidden_type < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: OpaqueTypeKey < 'tcx > , mut ty : ty :: OpaqueHiddenType < 'tcx > ,) -> Result < () , ErrorGuaranteed > { if ty . ty . is_ty_var () { return Ok (()) ; } if let ty :: Alias (ty :: Opaque , alias) = ty . ty . kind () { if alias . def_id == key . def_id . to_def_id () && alias . args == key . args { return Ok (()) ; } } let strip_vars = | ty : Ty < 'tcx > | { ty . fold_with (& mut BottomUpFolder { tcx , ty_op : | t | t , ct_op : | c | c , lt_op : | l | match l . kind () { RegionKind :: ReVar (_) => tcx . lifetimes . re_erased , _ => l , } , }) } ; ty . ty = strip_vars (ty . ty) ; let hidden_ty = tcx . type_of (key . def_id) . instantiate (tcx , key . args) ; let hidden_ty = strip_vars (hidden_ty) ; if hidden_ty == ty . ty { Ok (()) } else { let span = tcx . def_span (key . def_id) ; let other = ty :: OpaqueHiddenType { ty : hidden_ty , span } ; Err (ty . build_mismatch_error (& other , tcx) ? . emit ()) } }
    };
}

sanity_check_found_hidden_type!()