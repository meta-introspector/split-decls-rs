macro_rules! deps {
    () => {
        ValuePairs!();
        TypeTrace!();
        ToTrace!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: TraitRef < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: TraitRefs (ExpectedFound :: new (a , b)) } } }
    };
}

impl_13!();