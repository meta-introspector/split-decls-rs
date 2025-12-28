macro_rules! deps {
    () => {
        TypeTrace!();
        ValuePairs!();
        ToTrace!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: Region < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Regions (ExpectedFound :: new (a , b)) } } }
    };
}

impl_9!();