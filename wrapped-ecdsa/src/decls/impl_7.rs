macro_rules! deps {
    () => {
        SignatureSize!();
        SigningKey!();
        EcdsaCurve!();
        RecoveryId!();
        Signature!();
        DigestAlgorithm!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl < C , D > DigestSigner < D , (Signature < C > , RecoveryId) > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , D : EagerHash + digest :: Update , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign_digest < F : Fn (& mut D) -> Result < () > > (& self , f : F ,) -> Result < (Signature < C > , RecoveryId) > { let mut digest = D :: new () ; f (& mut digest) ? ; self . sign_digest_recoverable (digest) } }
    };
}

impl_7!();