// Generated macro for impl_287 (impl)
macro_rules! Depcrateimpl_287 {
() => {
// Module: crate
// Provides: {"impl_287"}
// Dependencies: {}
# [doc = " The `DigestSigner` implementation for `KeyPair` uses the optional deterministic variant of ML-DSA"] # [doc = " with a pre-computed μ, and only supports signing with an empty context string."] impl < P : MlDsaParams > DigestSigner < Shake256 , Signature < P > > for KeyPair < P > { fn try_sign_digest < F : Fn (& mut Shake256) -> Result < () , Error > > (& self , f : F ,) -> Result < Signature < P > , Error > { self . signing_key . try_sign_digest (& f) } }
};
}
