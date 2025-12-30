// Generated macro for impl_298 (impl)
macro_rules! Depcrateimpl_298 {
() => {
// Module: crate
// Provides: {"impl_298"}
// Dependencies: {}
# [doc = " The `RandomizedSigner` implementation for `SigningKey` only supports signing with an empty"] # [doc = " context string. If you would like to include a context string, use the"] # [doc = " [`SigningKey::sign_randomized`] method."] # [cfg (feature = "rand_core")] impl < P : MlDsaParams > RandomizedMultipartSigner < Signature < P > > for SigningKey < P > { fn try_multipart_sign_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [& [u8]] ,) -> Result < Signature < P > , Error > { self . raw_sign_randomized (msg , & [] , rng) } }
};
}
