macro_rules! deps {
    () => {
        SignatureSize!();
        EcdsaCurve!();
        SigningKey!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        # [cfg (feature = "pkcs8")] impl < C > AssociatedAlgorithmIdentifier for SigningKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { type Params = ObjectIdentifier ; const ALGORITHM_IDENTIFIER : AlgorithmIdentifier < ObjectIdentifier > = SecretKey :: < C > :: ALGORITHM_IDENTIFIER ; }
    };
}

impl_92!();