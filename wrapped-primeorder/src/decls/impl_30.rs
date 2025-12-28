macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < C > From < PublicKey < C > > for AffinePoint < C > where C : PrimeCurveParams , { fn from (public_key : PublicKey < C >) -> AffinePoint < C > { * public_key . as_affine () } }
    };
}

impl_30!();