macro_rules! deps {
    () => {
        DigestAlgorithm!();
        Signature!();
        RecoveryId!();
        EcdsaCurve!();
        SignatureSize!();
        SigningKey!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl < C > PrehashSigner < (Signature < C > , RecoveryId) > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn sign_prehash (& self , prehash : & [u8]) -> Result < (Signature < C > , RecoveryId) > { self . sign_prehash_recoverable (prehash) } }
    };
}

impl_10!()