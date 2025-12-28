macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < C > From < NonIdentity < AffinePoint < C > > > for AffinePoint < C > where C : PrimeCurveParams , { fn from (affine : NonIdentity < AffinePoint < C > >) -> Self { affine . to_point () } }
    };
}

impl_27!()