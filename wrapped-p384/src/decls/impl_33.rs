macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl elliptic_curve :: PrimeCurve for NistP384 { }
    };
}

impl_33!()