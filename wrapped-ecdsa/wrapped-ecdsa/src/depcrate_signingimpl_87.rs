// Generated macro for impl_87 (impl)
macro_rules! Depcrate_signingimpl_87 {
() => {
// Module: crate::signing
// Provides: {"impl_87"}
// Dependencies: {}
# [doc = " Sign message digest using a deterministic ephemeral scalar (`k`)"] # [doc = " computed using the algorithm described in [RFC6979 § 3.2]."] # [doc = ""] # [doc = " [RFC6979 § 3.2]: https://tools.ietf.org/html/rfc6979#section-3"] impl < C , D > DigestSigner < D , Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , D : EagerHash + Update , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign_digest < F : Fn (& mut D) -> Result < () > > (& self , f : F) -> Result < Signature < C > > { let mut digest = D :: new () ; f (& mut digest) ? ; self . sign_prehash (& digest . finalize ()) } }
};
}
