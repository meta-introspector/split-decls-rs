// Generated macro for impl_103 (impl)
macro_rules! Depcrate_ed25519impl_103 {
() => {
// Module: crate::ed25519
// Provides: {"impl_103"}
// Dependencies: {}
impl Deref for KeyPair { type Target = [u8 ; KeyPair :: BYTES] ; # [doc = " Returns a key pair as bytes."] fn deref (& self) -> & Self :: Target { & self . sk } }
};
}
