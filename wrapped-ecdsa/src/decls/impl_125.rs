macro_rules! deps {
    () => {
        VerifyingKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        # [cfg (feature = "pkcs8")] impl < C > AssociatedAlgorithmIdentifier for VerifyingKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { type Params = ObjectIdentifier ; const ALGORITHM_IDENTIFIER : AlgorithmIdentifier < ObjectIdentifier > = PublicKey :: < C > :: ALGORITHM_IDENTIFIER ; }
    };
}

impl_125!();