macro_rules! deps {
    () => {
        ImplTraitInTraitCollector!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < 'a , 'tcx , E > ImplTraitInTraitCollector < 'a , 'tcx , E > where E : 'tcx , { fn new (ocx : & 'a ObligationCtxt < 'a , 'tcx , E > , span : Span , param_env : ty :: ParamEnv < 'tcx > , body_id : LocalDefId ,) -> Self { ImplTraitInTraitCollector { ocx , types : FxIndexMap :: default () , span , param_env , body_id } } }
    };
}

impl_45!();