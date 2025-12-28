macro_rules! deps {
    () => {
        TypeTrace!();
        ValuePairs!();
        ToTrace!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: AliasTerm < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Aliases (ExpectedFound :: new (a , b)) } } }
    };
}

impl_15!();