macro_rules! deps {
    () => {
        EcdsaCurve!();
        VerifyingKey!();
        SignatureSize!();
    };
}

macro_rules! SigningKey {
    () => {
        deps!();
        # [doc = " ECDSA secret key used for signing. Generic over prime order elliptic curves"] # [doc = " (e.g. NIST P-curves)."] # [doc = ""] # [doc = " Requires an [`elliptic_curve::CurveArithmetic`] impl on the curve."] # [doc = ""] # [doc = " ## Usage"] # [doc = ""] # [doc = " The [`signature`] crate defines the following traits which are the"] # [doc = " primary API for signing:"] # [doc = ""] # [doc = " - [`Signer`]: sign a message using this key"] # [doc = " - [`DigestSigner`]: sign the output of a [`Digest`] using this key"] # [doc = " - [`PrehashSigner`]: sign the low-level raw output bytes of a message digest"] # [doc = ""] # [doc = " See the [`p256` crate](https://docs.rs/p256/latest/p256/ecdsa/index.html)"] # [doc = " for examples of using this type with a concrete elliptic curve."] # [derive (Clone)] pub struct SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { # [doc = " ECDSA signing keys are non-zero elements of a given curve's scalar field."] secret_scalar : NonZeroScalar < C > , # [doc = " Verifying key which corresponds to this signing key."] # [cfg (feature = "algorithm")] verifying_key : VerifyingKey < C > , }
    };
}

SigningKey!();