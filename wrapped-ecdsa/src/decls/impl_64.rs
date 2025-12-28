macro_rules! deps {
    () => {
        Signature!();
        EcdsaCurve!();
        SigningKey!();
        DigestAlgorithm!();
        SignatureSize!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < C > RandomizedSigner < Signature < C > > for SigningKey < C > where Self : RandomizedDigestSigner < C :: Digest , Signature < C > > , C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [u8] ,) -> Result < Signature < C > > { self . try_multipart_sign_with_rng (rng , & [msg]) } }
    };
}

impl_64!()