macro_rules! deps {
    () => {
        PredicateObligations!();
        LatticeOpKind!();
        InferCtxt!();
        TypeTrace!();
    };
}

macro_rules! LatticeOp {
    () => {
        deps!();
        # [doc = " A greatest lower bound\" (common subtype) or least upper bound (common supertype)."] pub (crate) struct LatticeOp < 'infcx , 'tcx > { infcx : & 'infcx InferCtxt < 'tcx > , trace : TypeTrace < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , kind : LatticeOpKind , obligations : PredicateObligations < 'tcx > , }
    };
}

LatticeOp!();