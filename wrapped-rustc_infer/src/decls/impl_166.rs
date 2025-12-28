macro_rules! deps {
    () => {
        LatticeOp!();
        TypeTrace!();
        InferCtxt!();
        LatticeOpKind!();
        PredicateObligations!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < 'infcx , 'tcx > LatticeOp < 'infcx , 'tcx > { pub (crate) fn new (infcx : & 'infcx InferCtxt < 'tcx > , trace : TypeTrace < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , kind : LatticeOpKind ,) -> LatticeOp < 'infcx , 'tcx > { LatticeOp { infcx , trace , param_env , kind , obligations : PredicateObligations :: new () } } pub (crate) fn into_obligations (self) -> PredicateObligations < 'tcx > { self . obligations } }
    };
}

impl_166!()