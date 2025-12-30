// Generated macro for impl_36 (impl)
macro_rules! Depcrate_pkcs8impl_36 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_36"}
// Dependencies: {}
# [cfg (feature = "pem")] impl str :: FromStr for PublicKeyBytes { type Err = spki :: Error ; fn from_str (pem : & str) -> spki :: Result < Self > { Self :: from_public_key_pem (pem) } }
};
}
