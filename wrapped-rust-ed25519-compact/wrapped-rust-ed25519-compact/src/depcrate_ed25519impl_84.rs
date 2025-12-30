// Generated macro for impl_84 (impl)
macro_rules! Depcrate_ed25519impl_84 {
() => {
// Module: crate::ed25519
// Provides: {"impl_84"}
// Dependencies: {}
impl Deref for SecretKey { type Target = [u8 ; SecretKey :: BYTES] ; # [doc = " Returns a secret key as bytes."] fn deref (& self) -> & Self :: Target { & self . 0 } }
};
}
