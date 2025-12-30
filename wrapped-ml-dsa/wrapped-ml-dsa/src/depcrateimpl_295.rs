// Generated macro for impl_295 (impl)
macro_rules! Depcrateimpl_295 {
() => {
// Module: crate
// Provides: {"impl_295"}
// Dependencies: {}
# [doc = " The `Signer` implementation for `SigningKey` uses the optional deterministic variant of ML-DSA"] # [doc = " with a pre-computed µ, and only supports signing with an empty context string. If you would"] # [doc = " like to include a context string, use the [`SigningKey::sign_mu_deterministic`] method."] impl < P : MlDsaParams > DigestSigner < Shake256 , Signature < P > > for SigningKey < P > { fn try_sign_digest < F : Fn (& mut Shake256) -> Result < () , Error > > (& self , f : F ,) -> Result < Signature < P > , Error > { let mut mu = MuBuilder :: new (& self . tr , & []) ; f (mu . as_mut ()) ? ; let mu = mu . finish () ; Ok (self . sign_mu_deterministic (& mu)) } }
};
}
