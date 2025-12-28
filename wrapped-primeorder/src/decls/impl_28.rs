macro_rules! deps {
    () => {
        AffinePoint!();
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < C > From < ProjectivePoint < C > > for AffinePoint < C > where C : PrimeCurveParams , { fn from (p : ProjectivePoint < C >) -> AffinePoint < C > { p . to_affine () } }
    };
}

impl_28!();