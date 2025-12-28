macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < C > Eq for ProjectivePoint < C > where C : PrimeCurveParams { }
    };
}

impl_59!()