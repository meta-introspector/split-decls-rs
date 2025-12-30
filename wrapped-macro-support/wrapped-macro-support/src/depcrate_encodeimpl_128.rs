// Generated macro for impl_128 (impl)
macro_rules! Depcrate_encodeimpl_128 {
() => {
// Module: crate::encode
// Provides: {"impl_128"}
// Dependencies: {}
impl Encode for & [u8] { fn encode (& self , dst : & mut Encoder) { self . len () . encode (dst) ; dst . extend_from_slice (self) ; } }
};
}
