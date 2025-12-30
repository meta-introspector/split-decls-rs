// Generated macro for impl_358 (impl)
macro_rules! Depcrate_cryptoimpl_358 {
() => {
// Module: crate::crypto
// Provides: {"impl_358"}
// Dependencies: {}
impl HeaderProtectionKey { pub fn from_secret (aead : Algorithm , secret : & [u8]) -> Result < Self > { let key_len = aead . key_len () ; let mut hp_key = vec ! [0 ; key_len] ; derive_hdr_key (aead , secret , & mut hp_key) ? ; Self :: new (aead , hp_key) } }
};
}
