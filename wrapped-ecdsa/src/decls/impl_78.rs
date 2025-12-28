macro_rules! deps {
    () => {
        EcdsaCurve!();
        SignatureSize!();
        SigningKey!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < C > Debug for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SigningKey") . finish_non_exhaustive () } }
    };
}

impl_78!()