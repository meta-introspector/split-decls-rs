macro_rules! deps {
    () => {
        VerifyingKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < C > TryFrom < & [u8] > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { type Error = Error ; fn try_from (bytes : & [u8]) -> Result < Self > { Self :: from_sec1_bytes (bytes) } }
    };
}

impl_124!()