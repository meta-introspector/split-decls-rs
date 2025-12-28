macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < C > TryFrom < ProjectivePoint < C > > for PublicKey < C > where C : PrimeCurveParams , { type Error = Error ; fn try_from (point : ProjectivePoint < C >) -> Result < PublicKey < C > > { AffinePoint :: < C > :: from (point) . try_into () } }
    };
}

impl_83!();