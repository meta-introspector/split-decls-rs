macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < C > TryFrom < AffinePoint < C > > for PublicKey < C > where C : PrimeCurveParams , { type Error = Error ; fn try_from (affine_point : AffinePoint < C >) -> Result < PublicKey < C > > { PublicKey :: from_affine (affine_point) } }
    };
}

impl_41!()