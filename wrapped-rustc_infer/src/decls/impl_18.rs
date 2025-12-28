macro_rules! deps {
    () => {
        TypeTrace!();
        ToTrace!();
        ValuePairs!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: PolyExistentialTraitRef < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: ExistentialTraitRef (ExpectedFound :: new (a , b)) , } } }
    };
}

impl_18!();