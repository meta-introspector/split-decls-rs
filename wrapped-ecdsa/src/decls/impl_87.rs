macro_rules! deps {
    () => {
        SigningKey!();
        EcdsaCurve!();
        SignatureSize!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < C > TryFrom < & [u8] > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { type Error = Error ; fn try_from (bytes : & [u8]) -> Result < Self > { Self :: from_slice (bytes) } }
    };
}

impl_87!()