macro_rules! deps {
    () => {
        RecoveryId!();
        SigningKey!();
        DigestAlgorithm!();
        EcdsaCurve!();
        Signature!();
        SignatureSize!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl < C , D > RandomizedDigestSigner < D , (Signature < C > , RecoveryId) > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , D : EagerHash + digest :: Update , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign_digest_with_rng < R : TryCryptoRng + ? Sized , F : Fn (& mut D) -> Result < () > > (& self , rng : & mut R , f : F ,) -> Result < (Signature < C > , RecoveryId) > { let mut digest = D :: new () ; f (& mut digest) ? ; self . sign_prehash_with_rng (rng , & digest . finalize ()) } }
    };
}

impl_9!()