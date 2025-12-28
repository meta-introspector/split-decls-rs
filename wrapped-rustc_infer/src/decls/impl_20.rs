macro_rules! deps {
    () => {
        ValuePairs!();
        TypeTrace!();
        ToTrace!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: PolyExistentialProjection < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: ExistentialProjection (ExpectedFound :: new (a , b)) , } } }
    };
}

impl_20!();