macro_rules! deps {
    () => {
        RecoveryId!();
        SignatureSize!();
        EcdsaCurve!();
        DigestAlgorithm!();
        Signature!();
        SigningKey!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl < C > MultipartSigner < (Signature < C > , RecoveryId) > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_multipart_sign (& self , msg : & [& [u8]]) -> Result < (Signature < C > , RecoveryId) > { let mut digest = C :: Digest :: new () ; msg . iter () . for_each (| slice | digest . update (slice)) ; self . sign_digest_recoverable (digest) } }
    };
}

impl_12!()