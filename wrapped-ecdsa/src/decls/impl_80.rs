macro_rules! deps {
    () => {
        SignatureSize!();
        SigningKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        # [doc = " Constant-time comparison"] impl < C > Eq for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { }
    };
}

impl_80!()