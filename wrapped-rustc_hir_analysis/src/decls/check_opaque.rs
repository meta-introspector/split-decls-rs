macro_rules! check_opaque {
    () => {
        # [doc = " Checks that an opaque type does not contain cycles and does not use `Self` or `T::Foo`"] # [doc = " projections that would result in \"inheriting lifetimes\"."] fn check_opaque (tcx : TyCtxt < '_ > , def_id : LocalDefId) { let hir :: OpaqueTy { origin , .. } = * tcx . hir_expect_opaque_ty (def_id) ; if tcx . sess . opts . actually_rustdoc { return ; } if tcx . type_of (def_id) . instantiate_identity () . references_error () { return ; } if check_opaque_for_cycles (tcx , def_id) . is_err () { return ; } let _ = check_opaque_meets_bounds (tcx , def_id , origin) ; }
    };
}

check_opaque!();