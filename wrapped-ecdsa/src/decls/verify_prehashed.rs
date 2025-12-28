macro_rules! deps {
    () => {
        SignatureSize!();
        Signature!();
        EcdsaCurve!();
    };
}

macro_rules! verify_prehashed {
    () => {
        deps!();
        # [doc = " Verify the prehashed message against the provided ECDSA signature."] # [doc = ""] # [doc = " Accepts the following arguments:"] # [doc = ""] # [doc = " - `q`: public key with which to verify the signature."] # [doc = " - `z`: message digest to be verified. MUST BE OUTPUT OF A CRYPTOGRAPHICALLY SECURE DIGEST"] # [doc = "   ALGORITHM!!!"] # [doc = " - `sig`: signature to be verified against the key and message."] # [doc = ""] # [doc = " # Low-S Normalization"] # [doc = ""] # [doc = " This is a low-level function that does *NOT* apply the `EcdsaCurve::NORMALIZE_S` checks."] # [cfg (feature = "algorithm")] pub fn verify_prehashed < C > (q : & ProjectivePoint < C > , z : & FieldBytes < C > , sig : & Signature < C > ,) -> Result < () > where C : EcdsaCurve + CurveArithmetic , SignatureSize < C > : ArraySize , { let z = Scalar :: < C > :: reduce (z) ; let (r , s) = sig . split_scalars () ; let s_inv = * s . invert_vartime () ; let u1 = z * s_inv ; let u2 = * r * s_inv ; let x = ProjectivePoint :: < C > :: lincomb (& [(ProjectivePoint :: < C > :: generator () , u1) , (* q , u2)]) . to_affine () . x () ; if * r == Scalar :: < C > :: reduce (& x) { Ok (()) } else { Err (Error :: new ()) } }
    };
}

verify_prehashed!()