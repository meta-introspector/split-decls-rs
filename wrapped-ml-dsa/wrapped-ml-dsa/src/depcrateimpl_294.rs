// Generated macro for impl_294 (impl)
macro_rules! Depcrateimpl_294 {
() => {
// Module: crate
// Provides: {"impl_294"}
// Dependencies: {}
# [doc = " The `Signer` implementation for `SigningKey` uses the optional deterministic variant of ML-DSA, and"] # [doc = " only supports signing with an empty context string. If you would like to include a context"] # [doc = " string, use the [`SigningKey::sign_deterministic`] method."] impl < P : MlDsaParams > MultipartSigner < Signature < P > > for SigningKey < P > { fn try_multipart_sign (& self , msg : & [& [u8]]) -> Result < Signature < P > , Error > { self . raw_sign_deterministic (msg , & []) } }
};
}
