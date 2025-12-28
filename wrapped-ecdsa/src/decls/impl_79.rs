macro_rules! deps {
    () => {
        SignatureSize!();
        EcdsaCurve!();
        SigningKey!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < C > Drop for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn drop (& mut self) { self . secret_scalar . zeroize () ; } }
    };
}

impl_79!();