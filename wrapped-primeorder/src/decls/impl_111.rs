macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < C > Neg for ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn neg (self) -> ProjectivePoint < C > { ProjectivePoint :: neg (& self) } }
    };
}

impl_111!();