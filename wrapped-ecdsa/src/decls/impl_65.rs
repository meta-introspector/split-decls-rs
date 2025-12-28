macro_rules! deps {
    () => {
        SigningKey!();
        SignatureSize!();
        Signature!();
        DigestAlgorithm!();
        EcdsaCurve!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < C > RandomizedMultipartSigner < Signature < C > > for SigningKey < C > where Self : RandomizedDigestSigner < C :: Digest , Signature < C > > , C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_multipart_sign_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [& [u8]] ,) -> Result < Signature < C > > { self . try_sign_digest_with_rng (rng , | digest : & mut C :: Digest | { msg . iter () . for_each (| slice | digest . update (slice)) ; Ok (()) }) } }
    };
}

impl_65!()