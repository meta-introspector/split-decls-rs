// Generated macro for impl_285 (impl)
macro_rules! Depcrateimpl_285 {
() => {
// Module: crate
// Provides: {"impl_285"}
// Dependencies: {}
# [doc = " The `Signer` implementation for `KeyPair` uses the optional deterministic variant of ML-DSA, and"] # [doc = " only supports signing with an empty context string."] impl < P : MlDsaParams > Signer < Signature < P > > for KeyPair < P > { fn try_sign (& self , msg : & [u8]) -> Result < Signature < P > , Error > { self . try_multipart_sign (& [msg]) } }
};
}
