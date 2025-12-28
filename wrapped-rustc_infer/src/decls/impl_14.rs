macro_rules! deps {
    () => {
        ToTrace!();
        TypeTrace!();
        ValuePairs!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: AliasTy < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Aliases (ExpectedFound :: new (a . into () , b . into ())) , } } }
    };
}

impl_14!()