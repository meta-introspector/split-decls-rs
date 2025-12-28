macro_rules! deps {
    () => {
        ToTrace!();
        TypeTrace!();
        ValuePairs!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for Const < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Terms (ExpectedFound :: new (a . into () , b . into ())) , } } }
    };
}

impl_10!()