// Generated macro for impl_103 (impl)
macro_rules! Depcrate_agreementimpl_103 {
() => {
// Module: crate::agreement
// Provides: {"impl_103"}
// Dependencies: {}
impl AsBigEndian < Curve25519SeedBin < 'static > > for PrivateKey { # [doc = " Exposes the seed encoded as a big-endian fixed-length integer."] # [doc = ""] # [doc = " Only X25519 is supported."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if serialization failed."] fn as_be_bytes (& self) -> Result < Curve25519SeedBin < 'static > , Unspecified > { if AlgorithmID :: X25519 != self . inner_key . algorithm () . id { return Err (Unspecified) ; } let evp_pkey = self . inner_key . get_evp_pkey () ; Ok (Curve25519SeedBin :: new (evp_pkey . as_const () . marshal_raw_private_key () ? ,)) } }
};
}
