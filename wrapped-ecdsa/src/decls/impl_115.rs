macro_rules! deps {
    () => {
        EcdsaCurve!();
        VerifyingKey!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < C > From < & VerifyingKey < C > > for EncodedPoint < C > where C : EcdsaCurve + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn from (verifying_key : & VerifyingKey < C >) -> EncodedPoint < C > { verifying_key . inner . into () } }
    };
}

impl_115!()