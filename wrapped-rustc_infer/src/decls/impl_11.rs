macro_rules! deps {
    () => {
        TypeTrace!();
        ValuePairs!();
        ToTrace!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'tcx > ToTrace < 'tcx > for ty :: GenericArg < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : match (a . kind () , b . kind ()) { (GenericArgKind :: Lifetime (a) , GenericArgKind :: Lifetime (b)) => { ValuePairs :: Regions (ExpectedFound :: new (a , b)) } (GenericArgKind :: Type (a) , GenericArgKind :: Type (b)) => { ValuePairs :: Terms (ExpectedFound :: new (a . into () , b . into ())) } (GenericArgKind :: Const (a) , GenericArgKind :: Const (b)) => { ValuePairs :: Terms (ExpectedFound :: new (a . into () , b . into ())) } _ => bug ! ("relating different kinds: {a:?} {b:?}") , } , } } }
    };
}

impl_11!()