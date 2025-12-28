macro_rules! deps {
    () => {
        EcdsaCurve!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl EcdsaCurve for MockCurve { const NORMALIZE_S : bool = false ; }
    };
}

impl_42!();