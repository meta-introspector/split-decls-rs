// Generated macro for impl_127 (impl)
macro_rules! Depcrate_encodeimpl_127 {
() => {
// Module: crate::encode
// Provides: {"impl_127"}
// Dependencies: {}
impl Encode for usize { fn encode (& self , dst : & mut Encoder) { assert ! (* self <= u32 :: MAX as usize) ; (* self as u32) . encode (dst) ; } }
};
}
