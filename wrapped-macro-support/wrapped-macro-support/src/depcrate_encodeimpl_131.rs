// Generated macro for impl_131 (impl)
macro_rules! Depcrate_encodeimpl_131 {
() => {
// Module: crate::encode
// Provides: {"impl_131"}
// Dependencies: {}
impl < T : Encode > Encode for Vec < T > { fn encode (& self , dst : & mut Encoder) { self . len () . encode (dst) ; for item in self { item . encode (dst) ; } } }
};
}
