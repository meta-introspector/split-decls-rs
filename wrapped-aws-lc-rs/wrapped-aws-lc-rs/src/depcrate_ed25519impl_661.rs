// Generated macro for impl_661 (impl)
macro_rules! Depcrate_ed25519impl_661 {
() => {
// Module: crate::ed25519
// Provides: {"impl_661"}
// Dependencies: {}
impl AsBigEndian < Curve25519SeedBin < 'static > > for Seed < '_ > { # [doc = " Exposes the seed encoded as a big-endian fixed-length integer."] # [doc = ""] # [doc = " For most use-cases, `EcdsaKeyPair::to_pkcs8()` should be preferred."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if serialization failed."] fn as_be_bytes (& self) -> Result < Curve25519SeedBin < 'static > , Unspecified > { Ok (Curve25519SeedBin :: new (self . bytes . to_vec ())) } }
};
}
