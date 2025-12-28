macro_rules! deps {
    () => {
        EcdsaCurve!();
        VerifyingKey!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        # [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl < C > EncodePublicKey for VerifyingKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn to_public_key_der (& self) -> spki :: Result < pkcs8 :: Document > { self . inner . to_public_key_der () } }
    };
}

impl_128!();