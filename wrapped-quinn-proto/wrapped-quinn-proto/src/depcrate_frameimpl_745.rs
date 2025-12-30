// Generated macro for impl_745 (impl)
macro_rules! Depcrate_frameimpl_745 {
() => {
// Module: crate::frame
// Provides: {"impl_745"}
// Dependencies: {}
impl Crypto { pub (crate) const SIZE_BOUND : usize = 17 ; pub (crate) fn encode < W : BufMut > (& self , out : & mut W) { out . write (FrameType :: CRYPTO) ; out . write_var (self . offset) ; out . write_var (self . data . len () as u64) ; out . put_slice (& self . data) ; } }
};
}
