macro_rules! deps {
    () => {
        SigningKey!();
        EcdsaCurve!();
        SignatureSize!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < C > ZeroizeOnDrop for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { }
    };
}

impl_88!()