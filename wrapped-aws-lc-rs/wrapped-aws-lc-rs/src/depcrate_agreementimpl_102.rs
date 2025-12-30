// Generated macro for impl_102 (impl)
macro_rules! Depcrate_agreementimpl_102 {
() => {
// Module: crate::agreement
// Provides: {"impl_102"}
// Dependencies: {}
impl AsBigEndian < EcPrivateKeyBin < 'static > > for PrivateKey { # [doc = " Exposes the private key encoded as a big-endian fixed-length integer."] # [doc = ""] # [doc = " X25519 is not supported."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if serialization failed."] fn as_be_bytes (& self) -> Result < EcPrivateKeyBin < 'static > , Unspecified > { if AlgorithmID :: X25519 == self . inner_key . algorithm () . id { return Err (Unspecified) ; } let buffer = marshal_sec1_private_key (self . inner_key . get_evp_pkey ()) ? ; Ok (EcPrivateKeyBin :: new (buffer)) } }
};
}
