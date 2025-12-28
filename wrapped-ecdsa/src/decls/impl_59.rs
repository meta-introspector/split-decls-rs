macro_rules! deps {
    () => {
        DigestAlgorithm!();
        EcdsaCurve!();
        SignatureSize!();
        SigningKey!();
        Signature!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        # [doc = " Sign message prehash using a deterministic ephemeral scalar (`k`)"] # [doc = " computed using the algorithm described in [RFC6979 § 3.2]."] # [doc = ""] # [doc = " [RFC6979 § 3.2]: https://tools.ietf.org/html/rfc6979#section-3"] impl < C > PrehashSigner < Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn sign_prehash (& self , prehash : & [u8]) -> Result < Signature < C > > { let z = bits2field :: < C > (prehash) ? ; Ok (sign_prehashed_rfc6979 :: < C , C :: Digest > (& self . secret_scalar , & z , & []) ? . 0) } }
    };
}

impl_59!()