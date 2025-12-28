macro_rules! deps {
    () => {
        SignatureSize!();
        EcdsaCurve!();
        RecoveryId!();
        Signature!();
        DigestAlgorithm!();
        SigningKey!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl < C > RandomizedPrehashSigner < (Signature < C > , RecoveryId) > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn sign_prehash_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , prehash : & [u8] ,) -> Result < (Signature < C > , RecoveryId) > { self . sign_prehash_recoverable_with_rng (rng , prehash) } }
    };
}

impl_8!()