macro_rules! ImplTraitInTraitCollector {
    () => {
        struct ImplTraitInTraitCollector < 'a , 'tcx , E > { ocx : & 'a ObligationCtxt < 'a , 'tcx , E > , types : FxIndexMap < DefId , (Ty < 'tcx > , ty :: GenericArgsRef < 'tcx >) > , span : Span , param_env : ty :: ParamEnv < 'tcx > , body_id : LocalDefId , }
    };
}

ImplTraitInTraitCollector!()