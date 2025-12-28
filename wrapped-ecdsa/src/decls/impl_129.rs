macro_rules! deps {
    () => {
        EcdsaCurve!();
        VerifyingKey!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        # [cfg (feature = "pem")] impl < C > FromStr for VerifyingKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { type Err = Error ; fn from_str (s : & str) -> Result < Self > { Self :: from_public_key_pem (s) . map_err (| _ | Error :: new ()) } }
    };
}

impl_129!()