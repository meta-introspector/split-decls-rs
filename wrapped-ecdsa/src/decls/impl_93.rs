macro_rules! deps {
    () => {
        EcdsaCurve!();
        SigningKey!();
        Signature!();
        SignatureSize!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [cfg (feature = "pkcs8")] impl < C > SignatureAlgorithmIdentifier for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , Signature < C > : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { type Params = AnyRef < 'static > ; const SIGNATURE_ALGORITHM_IDENTIFIER : AlgorithmIdentifier < Self :: Params > = Signature :: < C > :: ALGORITHM_IDENTIFIER ; }
    };
}

impl_93!()