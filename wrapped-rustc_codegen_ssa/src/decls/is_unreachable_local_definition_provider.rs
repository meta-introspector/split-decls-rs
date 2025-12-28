macro_rules! is_unreachable_local_definition_provider {
    () => {
        fn is_unreachable_local_definition_provider (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { ! tcx . reachable_set (()) . contains (& def_id) }
    };
}

is_unreachable_local_definition_provider!()