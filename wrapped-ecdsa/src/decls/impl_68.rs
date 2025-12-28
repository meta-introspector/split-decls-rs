macro_rules! deps {
    () => {
        EcdsaCurve!();
        SignatureWithOid!();
        SignatureSize!();
        DigestAlgorithm!();
        SigningKey!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < C > MultipartSigner < SignatureWithOid < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , C :: Digest : AssociatedOid , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_multipart_sign (& self , msg : & [& [u8]]) -> Result < SignatureWithOid < C > > { self . try_sign_digest (| digest : & mut C :: Digest | { msg . iter () . for_each (| slice | digest . update (slice)) ; Ok (()) }) } }
    };
}

impl_68!();