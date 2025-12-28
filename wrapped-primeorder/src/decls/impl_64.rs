macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < C > From < & PublicKey < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn from (public_key : & PublicKey < C >) -> ProjectivePoint < C > { AffinePoint :: < C > :: from (public_key) . into () } }
    };
}

impl_64!()