macro_rules! deps {
    () => {
        ValuePairs!();
        TypeTrace!();
        ToTrace!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: Term < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Terms (ExpectedFound :: new (a , b)) } } }
    };
}

impl_12!()