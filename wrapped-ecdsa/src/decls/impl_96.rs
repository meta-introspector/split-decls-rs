macro_rules! deps {
    () => {
        SignatureSize!();
        SigningKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        # [cfg (feature = "pem")] impl < C > FromStr for SigningKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { type Err = Error ; fn from_str (s : & str) -> Result < Self > { Self :: from_pkcs8_pem (s) . map_err (| _ | Error :: new ()) } }
    };
}

impl_96!();