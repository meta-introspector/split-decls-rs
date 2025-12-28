macro_rules! deps {
    () => {
        ToTrace!();
        ValuePairs!();
        TypeTrace!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: FnSig < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: PolySigs (ExpectedFound :: new (ty :: Binder :: dummy (a) , ty :: Binder :: dummy (b) ,)) , } } }
    };
}

impl_16!()