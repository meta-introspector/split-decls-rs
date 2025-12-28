macro_rules! upstream_drop_glue_for_provider {
    () => {
        fn upstream_drop_glue_for_provider < 'tcx > (tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > ,) -> Option < CrateNum > { let def_id = tcx . lang_items () . drop_in_place_fn () ? ; tcx . upstream_monomorphizations_for (def_id) ? . get (& args) . cloned () }
    };
}

upstream_drop_glue_for_provider!();