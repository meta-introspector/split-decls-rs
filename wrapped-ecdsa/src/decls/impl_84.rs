macro_rules! deps {
    () => {
        EcdsaCurve!();
        SigningKey!();
        SignatureSize!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < C > From < & SecretKey < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn from (secret_key : & SecretKey < C >) -> Self { secret_key . to_nonzero_scalar () . into () } }
    };
}

impl_84!()