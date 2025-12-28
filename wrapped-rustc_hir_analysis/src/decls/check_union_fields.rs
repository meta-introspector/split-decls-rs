macro_rules! deps {
    () => {
        InvalidUnionField!();
        InvalidUnionFieldSuggestion!();
    };
}

macro_rules! check_union_fields {
    () => {
        deps!();
        # [doc = " Check that the fields of the `union` do not need dropping."] fn check_union_fields (tcx : TyCtxt < '_ > , span : Span , item_def_id : LocalDefId) -> bool { let def = tcx . adt_def (item_def_id) ; assert ! (def . is_union ()) ; let typing_env = ty :: TypingEnv :: non_body_analysis (tcx , item_def_id) ; let args = ty :: GenericArgs :: identity_for_item (tcx , item_def_id) ; for field in & def . non_enum_variant () . fields { if ! allowed_union_or_unsafe_field (tcx , field . ty (tcx , args) , typing_env , span) { let (field_span , ty_span) = match tcx . hir_get_if_local (field . did) { Some (Node :: Field (field)) => (field . span , field . ty . span) , _ => unreachable ! ("mir field has to correspond to hir field") , } ; tcx . dcx () . emit_err (errors :: InvalidUnionField { field_span , sugg : errors :: InvalidUnionFieldSuggestion { lo : ty_span . shrink_to_lo () , hi : ty_span . shrink_to_hi () , } , note : () , }) ; return false ; } } true }
    };
}

check_union_fields!();