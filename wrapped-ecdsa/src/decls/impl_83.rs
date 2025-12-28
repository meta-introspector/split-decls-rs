macro_rules! deps {
    () => {
        SignatureSize!();
        EcdsaCurve!();
        SigningKey!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < C > From < SecretKey < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn from (secret_key : SecretKey < C >) -> Self { Self :: from (& secret_key) } }
    };
}

impl_83!();