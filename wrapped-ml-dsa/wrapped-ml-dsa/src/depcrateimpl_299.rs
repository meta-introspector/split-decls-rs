// Generated macro for impl_299 (impl)
macro_rules! Depcrateimpl_299 {
() => {
// Module: crate
// Provides: {"impl_299"}
// Dependencies: {}
# [doc = " The `RandomizedSigner` implementation for `SigningKey` only supports signing with an empty"] # [doc = " context string. If you would like to include a context string, use the"] # [doc = " [`SigningKey::sign_mu_randomized`] method."] # [cfg (feature = "rand_core")] impl < P : MlDsaParams > RandomizedDigestSigner < Shake256 , Signature < P > > for SigningKey < P > { fn try_sign_digest_with_rng < R : TryCryptoRng + ? Sized , F : Fn (& mut Shake256) -> Result < () , Error > , > (& self , rng : & mut R , f : F ,) -> Result < Signature < P > , Error > { let mut mu = MuBuilder :: new (& self . tr , & []) ; f (mu . as_mut ()) ? ; let mu = mu . finish () ; self . sign_mu_randomized (& mu , rng) } }
};
}
