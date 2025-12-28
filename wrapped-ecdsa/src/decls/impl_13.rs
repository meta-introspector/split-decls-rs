macro_rules! deps {
    () => {
        VerifyingKey!();
        DigestAlgorithm!();
        SignatureSize!();
        EcdsaCurve!();
        RecoveryId!();
        Signature!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl < C > VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , AffinePoint < C > : DecompressPoint < C > + FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , SignatureSize < C > : ArraySize , { # [doc = " Recover a [`VerifyingKey`] from the given message, signature, and"] # [doc = " [`RecoveryId`]."] # [doc = ""] # [doc = " The message is first hashed using this curve's [`DigestAlgorithm`]."] pub fn recover_from_msg (msg : & [u8] , signature : & Signature < C > , recovery_id : RecoveryId ,) -> Result < Self > where C : DigestAlgorithm , { Self :: recover_from_digest (C :: Digest :: new_with_prefix (msg) , signature , recovery_id) } # [doc = " Recover a [`VerifyingKey`] from the given message [`Digest`],"] # [doc = " signature, and [`RecoveryId`]."] pub fn recover_from_digest < D > (msg_digest : D , signature : & Signature < C > , recovery_id : RecoveryId ,) -> Result < Self > where D : EagerHash , { Self :: recover_from_prehash (& msg_digest . finalize () , signature , recovery_id) } # [doc = " Recover a [`VerifyingKey`] from the given `prehash` of a message, the"] # [doc = " signature over that prehashed message, and a [`RecoveryId`]."] pub fn recover_from_prehash (prehash : & [u8] , signature : & Signature < C > , recovery_id : RecoveryId ,) -> Result < Self > { let vk = Self :: recover_from_prehash_noverify (prehash , signature , recovery_id) ? ; verify_prehashed :: < C > (& ProjectivePoint :: < C > :: from (* vk . as_affine ()) , & bits2field :: < C > (prehash) ? , signature ,) ? ; Ok (vk) } # [doc = " Recover a [`VerifyingKey`] from the given `prehash` of a message, the"] # [doc = " signature over that prehashed message, and a [`RecoveryId`]. Compared to"] # [doc = " `recover_from_prehash`, this function skips verification with the"] # [doc = " recovered key."] # [allow (non_snake_case)] pub fn recover_from_prehash_noverify (prehash : & [u8] , signature : & Signature < C > , recovery_id : RecoveryId ,) -> Result < Self > { let (r , s) = signature . split_scalars () ; let z = Scalar :: < C > :: reduce (& bits2field :: < C > (prehash) ?) ; let r_bytes = if recovery_id . is_x_reduced () { C :: Uint :: decode_field_bytes (& r . to_repr ()) . checked_add (& C :: ORDER) . into_option () . ok_or_else (Error :: new) ? . encode_field_bytes () } else { r . to_repr () } ; let R : ProjectivePoint < C > = AffinePoint :: < C > :: decompress (& r_bytes , u8 :: from (recovery_id . is_y_odd ()) . into ()) . into_option () . ok_or_else (Error :: new) ? . into () ; let r_inv = * r . invert () ; let u1 = - (r_inv * z) ; let u2 = r_inv * * s ; let pk = ProjectivePoint :: < C > :: lincomb (& [(ProjectivePoint :: < C > :: generator () , u1) , (R , u2)]) ; Self :: from_affine (pk . into ()) } }
    };
}

impl_13!()