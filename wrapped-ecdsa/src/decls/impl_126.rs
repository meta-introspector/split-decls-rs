macro_rules! deps {
    () => {
        Signature!();
        VerifyingKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        # [cfg (feature = "pkcs8")] impl < C > SignatureAlgorithmIdentifier for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , Signature < C > : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { type Params = AnyRef < 'static > ; const SIGNATURE_ALGORITHM_IDENTIFIER : AlgorithmIdentifier < Self :: Params > = Signature :: < C > :: ALGORITHM_IDENTIFIER ; }
    };
}

impl_126!();