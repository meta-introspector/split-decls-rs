macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < C > From < NonIdentity < ProjectivePoint < C > > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn from (p : NonIdentity < ProjectivePoint < C > >) -> Self { p . to_point () } }
    };
}

impl_62!();