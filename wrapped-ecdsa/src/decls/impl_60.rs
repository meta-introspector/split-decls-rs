macro_rules! deps {
    () => {
        DigestAlgorithm!();
        SigningKey!();
        SignatureSize!();
        EcdsaCurve!();
        Signature!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        # [doc = " Sign message using a deterministic ephemeral scalar (`k`)"] # [doc = " computed using the algorithm described in [RFC6979 § 3.2]."] # [doc = ""] # [doc = " [RFC6979 § 3.2]: https://tools.ietf.org/html/rfc6979#section-3"] impl < C > Signer < Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign (& self , msg : & [u8]) -> Result < Signature < C > > { self . try_multipart_sign (& [msg]) } }
    };
}

impl_60!()