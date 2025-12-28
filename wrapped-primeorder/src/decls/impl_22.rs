macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < C > DefaultIsZeroes for AffinePoint < C > where C : PrimeCurveParams { }
    };
}

impl_22!()