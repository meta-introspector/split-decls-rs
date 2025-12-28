macro_rules! deps {
    () => {
        SignatureSize!();
        Signature!();
        DigestAlgorithm!();
        SigningKey!();
        EcdsaCurve!();
        RecoveryId!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl < C > SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { # [doc = " Sign the given message prehash, using the given rng for the RFC6979 Section 3.6 \"additional"] # [doc = " data\", returning a signature and recovery ID."] pub fn sign_prehash_recoverable_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , prehash : & [u8] ,) -> Result < (Signature < C > , RecoveryId) > { let z = bits2field :: < C > (prehash) ? ; loop { let mut ad = FieldBytes :: < C > :: default () ; rng . try_fill_bytes (& mut ad) . map_err (| _ | Error :: new ()) ? ; if let Ok (result) = sign_prehashed_rfc6979 :: < C , C :: Digest > (self . as_nonzero_scalar () , & z , & ad) { break Ok (result) ; } } } # [doc = " Sign the given message prehash, returning a signature and recovery ID."] pub fn sign_prehash_recoverable (& self , prehash : & [u8]) -> Result < (Signature < C > , RecoveryId) > { let z = bits2field :: < C > (prehash) ? ; sign_prehashed_rfc6979 :: < C , C :: Digest > (self . as_nonzero_scalar () , & z , & []) } # [doc = " Sign the given message digest, returning a signature and recovery ID."] pub fn sign_digest_recoverable < D > (& self , msg_digest : D) -> Result < (Signature < C > , RecoveryId) > where D : EagerHash , { self . sign_prehash_recoverable (& msg_digest . finalize ()) } # [doc = " Sign the given message, hashing it with the curve's default digest"] # [doc = " function, and returning a signature and recovery ID."] pub fn sign_recoverable (& self , msg : & [u8]) -> Result < (Signature < C > , RecoveryId) > { self . sign_digest_recoverable (C :: Digest :: new_with_prefix (msg)) } }
    };
}

impl_6!()