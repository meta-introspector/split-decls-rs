macro_rules! deps {
    () => {
        DigestAlgorithm!();
        RecoveryId!();
        Signature!();
        EcdsaCurve!();
        VerifyingKey!();
        SignatureSize!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl RecoveryId { # [doc = " Given a public key, message, and signature, use trial recovery"] # [doc = " to determine if a suitable recovery ID exists, or return an error"] # [doc = " otherwise."] pub fn trial_recovery_from_msg < C > (verifying_key : & VerifyingKey < C > , msg : & [u8] , signature : & Signature < C > ,) -> Result < Self > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , AffinePoint < C > : DecompressPoint < C > + FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , SignatureSize < C > : ArraySize , { Self :: trial_recovery_from_digest (verifying_key , C :: Digest :: new_with_prefix (msg) , signature) } # [doc = " Given a public key, message digest, and signature, use trial recovery"] # [doc = " to determine if a suitable recovery ID exists, or return an error"] # [doc = " otherwise."] pub fn trial_recovery_from_digest < C , D > (verifying_key : & VerifyingKey < C > , digest : D , signature : & Signature < C > ,) -> Result < Self > where C : EcdsaCurve + CurveArithmetic , D : EagerHash , AffinePoint < C > : DecompressPoint < C > + FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , SignatureSize < C > : ArraySize , { Self :: trial_recovery_from_prehash (verifying_key , & digest . finalize () , signature) } # [doc = " Given a public key, message digest, and signature, use trial recovery"] # [doc = " to determine if a suitable recovery ID exists, or return an error"] # [doc = " otherwise."] pub fn trial_recovery_from_prehash < C > (verifying_key : & VerifyingKey < C > , prehash : & [u8] , signature : & Signature < C > ,) -> Result < Self > where C : EcdsaCurve + CurveArithmetic , AffinePoint < C > : DecompressPoint < C > + FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , SignatureSize < C > : ArraySize , { verify_prehashed :: < C > (& ProjectivePoint :: < C > :: from (* verifying_key . as_affine ()) , & bits2field :: < C > (prehash) ? , signature ,) ? ; for id in 0 ..= Self :: MAX { let recovery_id = RecoveryId (id) ; if let Ok (vk) = VerifyingKey :: recover_from_prehash_noverify (prehash , signature , recovery_id) { if verifying_key == & vk { return Ok (recovery_id) ; } } } Err (Error :: new ()) } }
    };
}

impl_3!();