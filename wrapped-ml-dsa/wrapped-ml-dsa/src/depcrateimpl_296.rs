// Generated macro for impl_296 (impl)
macro_rules! Depcrateimpl_296 {
() => {
// Module: crate
// Provides: {"impl_296"}
// Dependencies: {}
# [doc = " The `KeyPair` implementation for `SigningKey` allows to derive a `VerifyingKey` from"] # [doc = " a bare `SigningKey` (even in the absence of the original seed)."] impl < P : MlDsaParams > signature :: Keypair for SigningKey < P > { type VerifyingKey = VerifyingKey < P > ; # [doc = " This is a utility function that is useful when importing the private key"] # [doc = " from an external source which does not export the seed and does not"] # [doc = " provide the precomputed public key associated with the private key"] # [doc = " itself."] fn verifying_key (& self) -> Self :: VerifyingKey { let As1 = & self . A_hat * & self . s1_hat ; let t = & As1 . ntt_inverse () + & self . s2 ; let (t1 , _) = t . power2round () ; VerifyingKey :: new (self . rho . clone () , t1 , Some (self . A_hat . clone ()) , None) } }
};
}
