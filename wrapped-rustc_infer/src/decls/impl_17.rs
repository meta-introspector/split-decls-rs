macro_rules! deps {
    () => {
        TypeTrace!();
        ValuePairs!();
        ToTrace!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: PolyFnSig < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: PolySigs (ExpectedFound :: new (a , b)) } } }
    };
}

impl_17!()