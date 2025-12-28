macro_rules! deps {
    () => {
        MaxSize!();
        DigestAlgorithm!();
        Signature!();
        MaxOverhead!();
        EcdsaCurve!();
        SigningKey!();
        SignatureSize!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        # [cfg (feature = "der")] impl < C , D > RandomizedDigestSigner < D , der :: Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , D : EagerHash + Update , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn try_sign_digest_with_rng < R : TryCryptoRng + ? Sized , F : Fn (& mut D) -> Result < () > > (& self , rng : & mut R , f : F ,) -> Result < der :: Signature < C > > { RandomizedDigestSigner :: < D , Signature < C > > :: try_sign_digest_with_rng (self , rng , f) . map (Into :: into) } }
    };
}

impl_71!()