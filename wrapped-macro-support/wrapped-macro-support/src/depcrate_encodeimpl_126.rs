// Generated macro for impl_126 (impl)
macro_rules! Depcrate_encodeimpl_126 {
() => {
// Module: crate::encode
// Provides: {"impl_126"}
// Dependencies: {}
impl Encode for u32 { fn encode (& self , dst : & mut Encoder) { let mut val = * self ; while (val >> 7) != 0 { dst . byte ((val as u8) | 0x80) ; val >>= 7 ; } assert_eq ! (val >> 7 , 0) ; dst . byte (val as u8) ; } }
};
}
