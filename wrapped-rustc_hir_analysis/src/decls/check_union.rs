macro_rules! check_union {
    () => {
        fn check_union (tcx : TyCtxt < '_ > , def_id : LocalDefId) { let def = tcx . adt_def (def_id) ; let span = tcx . def_span (def_id) ; def . destructor (tcx) ; check_transparent (tcx , def) ; check_union_fields (tcx , span , def_id) ; check_packed (tcx , span , def) ; }
    };
}

check_union!();