macro_rules! deps {
    () => {
        SignatureSize!();
        EcdsaCurve!();
        SigningKey!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        # [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl < C > EncodePrivateKey for SigningKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn to_pkcs8_der (& self) -> pkcs8 :: Result < SecretDocument > { SecretKey :: from (self . secret_scalar) . to_pkcs8_der () } }
    };
}

impl_95!()