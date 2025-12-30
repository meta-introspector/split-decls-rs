// Generated macro for impl_773 (impl)
macro_rules! Depcrate_hazardous_hpke_x25519_sha256_chacha20poly1305impl_773 {
() => {
// Module: crate::hazardous::hpke::x25519_sha256_chacha20poly1305
// Provides: {"impl_773"}
// Dependencies: {}
impl Drop for DHKEM_X25519_SHA256_CHACHA20 { fn drop (& mut self) { use zeroize :: Zeroize ; self . key . iter_mut () . zeroize () ; self . exporter_secret . iter_mut () . zeroize () ; } }
};
}
