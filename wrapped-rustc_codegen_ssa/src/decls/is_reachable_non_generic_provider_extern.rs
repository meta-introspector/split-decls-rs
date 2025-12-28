macro_rules! is_reachable_non_generic_provider_extern {
    () => {
        fn is_reachable_non_generic_provider_extern (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { tcx . reachable_non_generics (def_id . krate) . contains_key (& def_id) }
    };
}

is_reachable_non_generic_provider_extern!()