// Generated macro for impl_770 (impl)
macro_rules! Depcrate_hazardous_hpke_x25519_sha256_chacha20poly1305impl_770 {
() => {
// Module: crate::hazardous::hpke::x25519_sha256_chacha20poly1305
// Provides: {"impl_770"}
// Dependencies: {}
impl PartialEq < DHKEM_X25519_SHA256_CHACHA20 > for DHKEM_X25519_SHA256_CHACHA20 { fn eq (& self , other : & DHKEM_X25519_SHA256_CHACHA20) -> bool { use subtle :: ConstantTimeEq ; (self . key . ct_eq (& other . key) & self . base_nonce . ct_eq (& other . base_nonce) & self . ctr . ct_eq (& other . ctr) & self . exporter_secret . ct_eq (& other . exporter_secret)) . into () } }
};
}
