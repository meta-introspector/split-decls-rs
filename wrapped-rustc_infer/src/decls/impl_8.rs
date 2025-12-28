macro_rules! deps {
    () => {
        ToTrace!();
        ValuePairs!();
        TypeTrace!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for Ty < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Terms (ExpectedFound :: new (a . into () , b . into ())) , } } }
    };
}

impl_8!()