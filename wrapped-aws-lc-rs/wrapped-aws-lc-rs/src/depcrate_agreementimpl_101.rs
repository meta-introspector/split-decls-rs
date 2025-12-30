// Generated macro for impl_101 (impl)
macro_rules! Depcrate_agreementimpl_101 {
() => {
// Module: crate::agreement
// Provides: {"impl_101"}
// Dependencies: {}
impl AsDer < Pkcs8V1Der < 'static > > for PrivateKey { # [doc = " Serializes the key as a PKCS #8 private key structure."] # [doc = ""] # [doc = " X25519 is not supported."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified`  if serialization failed."] fn as_der (& self) -> Result < Pkcs8V1Der < 'static > , Unspecified > { if AlgorithmID :: X25519 == self . inner_key . algorithm () . id { return Err (Unspecified) ; } Ok (Pkcs8V1Der :: new (self . inner_key . get_evp_pkey () . as_const () . marshal_rfc5208_private_key (Version :: V1) ? ,)) } }
};
}
