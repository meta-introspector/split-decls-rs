macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
        ProjectivePoint!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < C > TryFrom < & ProjectivePoint < C > > for PublicKey < C > where C : PrimeCurveParams , { type Error = Error ; fn try_from (point : & ProjectivePoint < C >) -> Result < PublicKey < C > > { AffinePoint :: < C > :: from (point) . try_into () } }
    };
}

impl_84!()