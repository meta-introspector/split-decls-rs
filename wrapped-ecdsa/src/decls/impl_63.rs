macro_rules! deps {
    () => {
        SignatureSize!();
        SigningKey!();
        Signature!();
        EcdsaCurve!();
        DigestAlgorithm!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < C > RandomizedPrehashSigner < Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn sign_prehash_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , prehash : & [u8] ,) -> Result < Signature < C > > { let z = bits2field :: < C > (prehash) ? ; loop { let mut ad = FieldBytes :: < C > :: default () ; rng . try_fill_bytes (& mut ad) . map_err (| _ | Error :: new ()) ? ; if let Ok ((signature , _)) = sign_prehashed_rfc6979 :: < C , C :: Digest > (& self . secret_scalar , & z , & ad) { break Ok (signature) ; } } } }
    };
}

impl_63!();