macro_rules! used_trait_imports {
    () => {
        fn used_trait_imports (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> & UnordSet < LocalDefId > { & tcx . typeck (def_id) . used_trait_imports }
    };
}

used_trait_imports!();