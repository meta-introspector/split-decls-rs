macro_rules! deps {
    () => {
        DigestAlgorithm!();
        SigningKey!();
        EcdsaCurve!();
        SignatureWithOid!();
        SignatureSize!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < C > Signer < SignatureWithOid < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , C :: Digest : AssociatedOid , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign (& self , msg : & [u8]) -> Result < SignatureWithOid < C > > { self . try_multipart_sign (& [msg]) } }
    };
}

impl_67!();