macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < C > Eq for AffinePoint < C > where C : PrimeCurveParams { }
    };
}

impl_25!();