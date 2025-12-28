macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < C > DefaultIsZeroes for ProjectivePoint < C > where C : PrimeCurveParams { }
    };
}

impl_57!()