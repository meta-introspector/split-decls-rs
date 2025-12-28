macro_rules! deps {
    () => {
        ProjectivePoint!();
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < C > From < & ProjectivePoint < C > > for AffinePoint < C > where C : PrimeCurveParams , { fn from (p : & ProjectivePoint < C >) -> AffinePoint < C > { p . to_affine () } }
    };
}

impl_29!();