macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < C > From < & PublicKey < C > > for AffinePoint < C > where C : PrimeCurveParams , { fn from (public_key : & PublicKey < C >) -> AffinePoint < C > { AffinePoint :: from (* public_key) } }
    };
}

impl_31!()