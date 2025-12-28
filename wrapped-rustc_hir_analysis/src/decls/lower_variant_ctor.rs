macro_rules! lower_variant_ctor {
    () => {
        pub (super) fn lower_variant_ctor (tcx : TyCtxt < '_ > , def_id : LocalDefId) { tcx . ensure_ok () . generics_of (def_id) ; tcx . ensure_ok () . type_of (def_id) ; tcx . ensure_ok () . predicates_of (def_id) ; }
    };
}

lower_variant_ctor!()