macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < C > Default for ProjectivePoint < C > where C : PrimeCurveParams , { fn default () -> Self { Self :: IDENTITY } }
    };
}

impl_56!()