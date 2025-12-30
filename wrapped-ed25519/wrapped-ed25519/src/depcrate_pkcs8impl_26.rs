// Generated macro for impl_26 (impl)
macro_rules! Depcrate_pkcs8impl_26 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "pem")] impl str :: FromStr for KeypairBytes { type Err = Error ; fn from_str (pem : & str) -> Result < Self > { Self :: from_pkcs8_pem (pem) } }
};
}
