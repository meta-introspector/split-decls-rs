macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < C : PrimeCurveParams > Double for ProjectivePoint < C > { fn double (& self) -> Self { C :: PointArithmetic :: double (self) } }
    };
}

impl_58!();