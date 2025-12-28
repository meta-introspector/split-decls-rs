macro_rules! deps {
    () => {
        EcdsaCurve!();
        SignatureSize!();
        Signature!();
        RecoveryId!();
    };
}

macro_rules! sign_prehashed_rfc6979 {
    () => {
        deps!();
        # [doc = " Try to sign the given message digest deterministically using the method"] # [doc = " described in [RFC6979] for computing ECDSA ephemeral scalar `k`."] # [doc = ""] # [doc = " Accepts the following parameters:"] # [doc = " - `d`: signing key. MUST BE UNIFORMLY RANDOM!!!"] # [doc = " - `z`: message digest to be signed, i.e. `H(m)`. Does not have to be reduced in advance."] # [doc = " - `ad`: optional additional data, e.g. added entropy from an RNG"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This will return an error if a zero-scalar was generated. It can be tried again with different"] # [doc = " entropy `ad`."] # [doc = ""] # [doc = " [RFC6979]: https://datatracker.ietf.org/doc/html/rfc6979"] # [cfg (feature = "algorithm")] pub fn sign_prehashed_rfc6979 < C , D > (d : & NonZeroScalar < C > , z : & FieldBytes < C > , ad : & [u8] ,) -> Result < (Signature < C > , RecoveryId) > where C : EcdsaCurve + CurveArithmetic , D : EagerHash , SignatureSize < C > : ArraySize , { let z2 = Scalar :: < C > :: reduce (z) ; let k = NonZeroScalar :: < C > :: from_repr (rfc6979 :: generate_k :: < D , _ > (& d . to_repr () , & C :: ORDER . encode_field_bytes () , & z2 . to_repr () , ad ,)) . unwrap () ; sign_prehashed (d , & k , z) }
    };
}

sign_prehashed_rfc6979!();