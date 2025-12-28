macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
        AffinePoint!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < C > From < PublicKey < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn from (public_key : PublicKey < C >) -> ProjectivePoint < C > { AffinePoint :: from (public_key) . into () } }
    };
}

impl_63!()