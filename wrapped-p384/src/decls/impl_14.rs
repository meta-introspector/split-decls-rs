macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl EcdsaCurve for NistP384 { const NORMALIZE_S : bool = false ; }
    };
}

impl_14!()