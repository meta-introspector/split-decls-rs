macro_rules! deps {
    () => {
        SignatureSize!();
        VerifyingKey!();
        SigningKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl < C > KeypairRef for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { type VerifyingKey = VerifyingKey < C > ; }
    };
}

impl_91!()