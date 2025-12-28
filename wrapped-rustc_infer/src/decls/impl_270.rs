macro_rules! deps {
    () => {
        InferOk!();
        PredicateObligations!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < 'tcx > InferOk < 'tcx , () > { pub fn into_obligations (self) -> PredicateObligations < 'tcx > { self . obligations } }
    };
}

impl_270!();