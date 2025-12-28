macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < C > Default for AffinePoint < C > where C : PrimeCurveParams , { fn default () -> Self { Self :: IDENTITY } }
    };
}

impl_21!()