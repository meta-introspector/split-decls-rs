macro_rules! deps {
    () => {
        Locations!();
        TypeChecker!();
        UniverseInfo!();
    };
}

macro_rules! NllTypeRelating {
    () => {
        deps!();
        struct NllTypeRelating < 'a , 'b , 'tcx > { type_checker : & 'a mut TypeChecker < 'b , 'tcx > , # [doc = " Where (and why) is this relation taking place?"] locations : Locations , # [doc = " What category do we assign the resulting `'a: 'b` relationships?"] category : ConstraintCategory < 'tcx > , # [doc = " Information so that error reporting knows what types we are relating"] # [doc = " when reporting a bound region error."] universe_info : UniverseInfo < 'tcx > , # [doc = " How are we relating `a` and `b`?"] # [doc = ""] # [doc = " - Covariant means `a <: b`."] # [doc = " - Contravariant means `b <: a`."] # [doc = " - Invariant means `a == b`."] # [doc = " - Bivariant means that it doesn't matter."] ambient_variance : ty :: Variance , ambient_variance_info : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , }
    };
}

NllTypeRelating!()