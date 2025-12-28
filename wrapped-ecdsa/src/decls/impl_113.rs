macro_rules! deps {
    () => {
        VerifyingKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < C > From < & VerifyingKey < C > > for CompressedPoint < C > where C : EcdsaCurve + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn from (verifying_key : & VerifyingKey < C >) -> CompressedPoint < C > { verifying_key . inner . into () } }
    };
}

impl_113!();