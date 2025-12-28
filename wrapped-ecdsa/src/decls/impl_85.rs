macro_rules! deps {
    () => {
        EcdsaCurve!();
        SigningKey!();
        SignatureSize!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < C > From < SigningKey < C > > for SecretKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn from (key : SigningKey < C >) -> Self { key . secret_scalar . into () } }
    };
}

impl_85!();