macro_rules! deps {
    () => {
        ToTrace!();
        ValuePairs!();
        TypeTrace!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: ExistentialTraitRef < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: ExistentialTraitRef (ExpectedFound :: new (ty :: Binder :: dummy (a) , ty :: Binder :: dummy (b) ,)) , } } }
    };
}

impl_19!();