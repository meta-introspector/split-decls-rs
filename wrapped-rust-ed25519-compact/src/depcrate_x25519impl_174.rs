// Generated macro for impl_174 (impl)
macro_rules! Depcrate_x25519impl_174 {
() => {
// Module: crate::x25519
// Provides: {"impl_174"}
// Dependencies: {}
impl Deref for SecretKey { type Target = [u8 ; SecretKey :: BYTES] ; # [doc = " Returns a secret key as bytes."] fn deref (& self) -> & Self :: Target { & self . 0 } }
};
}
