// Generated macro for impl_730 (impl)
macro_rules! Depcrate_hazardous_kem_xwingimpl_730 {
() => {
// Module: crate::hazardous::kem::xwing
// Provides: {"impl_730"}
// Dependencies: {}
impl TryFrom < & Seed > for KeyPair { type Error = UnknownCryptoError ; fn try_from (value : & Seed) -> Result < Self , Self :: Error > { KeyPair :: generate_deterministic (value) } }
};
}
