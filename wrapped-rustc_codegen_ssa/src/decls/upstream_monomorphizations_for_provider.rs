macro_rules! upstream_monomorphizations_for_provider {
    () => {
        fn upstream_monomorphizations_for_provider (tcx : TyCtxt < '_ > , def_id : DefId ,) -> Option < & UnordMap < GenericArgsRef < '_ > , CrateNum > > { assert ! (! def_id . is_local ()) ; tcx . upstream_monomorphizations (()) . get (& def_id) }
    };
}

upstream_monomorphizations_for_provider!();