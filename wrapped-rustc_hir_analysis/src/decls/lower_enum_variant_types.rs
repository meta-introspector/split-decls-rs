macro_rules! deps {
    () => {
        EnumDiscriminantOverflowed!();
    };
}

macro_rules! lower_enum_variant_types {
    () => {
        deps!();
        pub (super) fn lower_enum_variant_types (tcx : TyCtxt < '_ > , def_id : DefId) { let def = tcx . adt_def (def_id) ; let repr_type = def . repr () . discr_type () ; let initial = repr_type . initial_discriminant (tcx) ; let mut prev_discr = None :: < Discr < '_ > > ; for variant in def . variants () { let wrapped_discr = prev_discr . map_or (initial , | d | d . wrap_incr (tcx)) ; prev_discr = Some (if let ty :: VariantDiscr :: Explicit (const_def_id) = variant . discr { def . eval_explicit_discr (tcx , const_def_id) . ok () } else if let Some (discr) = repr_type . disr_incr (tcx , prev_discr) { Some (discr) } else { let span = tcx . def_span (variant . def_id) ; tcx . dcx () . emit_err (errors :: EnumDiscriminantOverflowed { span , discr : prev_discr . unwrap () . to_string () , item_name : tcx . item_ident (variant . def_id) , wrapped_discr : wrapped_discr . to_string () , }) ; None } . unwrap_or (wrapped_discr) ,) ; for f in & variant . fields { tcx . ensure_ok () . generics_of (f . did) ; tcx . ensure_ok () . type_of (f . did) ; tcx . ensure_ok () . predicates_of (f . did) ; } if let Some (ctor_def_id) = variant . ctor_def_id () { lower_variant_ctor (tcx , ctor_def_id . expect_local ()) ; } } }
    };
}

lower_enum_variant_types!()