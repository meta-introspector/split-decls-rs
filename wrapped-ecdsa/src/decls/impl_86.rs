macro_rules! deps {
    () => {
        SignatureSize!();
        EcdsaCurve!();
        SigningKey!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < C > From < & SigningKey < C > > for SecretKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn from (secret_key : & SigningKey < C >) -> Self { secret_key . secret_scalar . into () } }
    };
}

impl_86!()