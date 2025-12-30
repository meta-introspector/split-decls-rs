// Generated macro for impl_293 (impl)
macro_rules! Depcrateimpl_293 {
() => {
// Module: crate
// Provides: {"impl_293"}
// Dependencies: {}
# [doc = " The `Signer` implementation for `SigningKey` uses the optional deterministic variant of ML-DSA, and"] # [doc = " only supports signing with an empty context string.  If you would like to include a context"] # [doc = " string, use the [`SigningKey::sign_deterministic`] method."] impl < P : MlDsaParams > Signer < Signature < P > > for SigningKey < P > { fn try_sign (& self , msg : & [u8]) -> Result < Signature < P > , Error > { self . try_multipart_sign (& [msg]) } }
};
}
