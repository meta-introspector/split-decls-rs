// Generated macro for impl_665 (impl)
macro_rules! Depcrate_ed25519impl_665 {
() => {
// Module: crate::ed25519
// Provides: {"impl_665"}
// Dependencies: {}
impl Debug for PublicKey { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_str (& format ! ("PublicKey(\"{}\")" , hex :: encode (self . public_key_bytes))) } }
};
}
