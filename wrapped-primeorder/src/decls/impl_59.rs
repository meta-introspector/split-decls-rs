macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < C > Eq for ProjectivePoint < C > where C : PrimeCurveParams { }
    };
}

impl_59!();