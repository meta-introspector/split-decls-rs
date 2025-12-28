macro_rules! deps {
    () => {
        ToTrace!();
        TypeTrace!();
        ValuePairs!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: ExistentialProjection < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: ExistentialProjection (ExpectedFound :: new (ty :: Binder :: dummy (a) , ty :: Binder :: dummy (b) ,)) , } } }
    };
}

impl_21!()