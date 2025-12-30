// Generated macro for impl_286 (impl)
macro_rules! Depcrateimpl_286 {
() => {
// Module: crate
// Provides: {"impl_286"}
// Dependencies: {}
# [doc = " The `Signer` implementation for `KeyPair` uses the optional deterministic variant of ML-DSA, and"] # [doc = " only supports signing with an empty context string."] impl < P : MlDsaParams > MultipartSigner < Signature < P > > for KeyPair < P > { fn try_multipart_sign (& self , msg : & [& [u8]]) -> Result < Signature < P > , Error > { self . signing_key . raw_sign_deterministic (msg , & []) } }
};
}
