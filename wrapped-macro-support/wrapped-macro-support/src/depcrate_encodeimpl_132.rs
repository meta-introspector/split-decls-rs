// Generated macro for impl_132 (impl)
macro_rules! Depcrate_encodeimpl_132 {
() => {
// Module: crate::encode
// Provides: {"impl_132"}
// Dependencies: {}
impl < T : Encode > Encode for Option < T > { fn encode (& self , dst : & mut Encoder) { match self { None => dst . byte (0) , Some (val) => { dst . byte (1) ; val . encode (dst) } } } }
};
}
