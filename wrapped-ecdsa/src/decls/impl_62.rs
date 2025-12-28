macro_rules! deps {
    () => {
        SignatureSize!();
        Signature!();
        EcdsaCurve!();
        DigestAlgorithm!();
        SigningKey!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < C , D > RandomizedDigestSigner < D , Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , D : EagerHash + Update , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign_digest_with_rng < R : TryCryptoRng + ? Sized , F : Fn (& mut D) -> Result < () > > (& self , rng : & mut R , f : F ,) -> Result < Signature < C > > { let mut digest = D :: new () ; f (& mut digest) ? ; self . sign_prehash_with_rng (rng , & digest . finalize ()) } }
    };
}

impl_62!()