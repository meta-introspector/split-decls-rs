// Generated macro for impl_747 (impl)
macro_rules! Depcrate_frameimpl_747 {
() => {
// Module: crate::frame
// Provides: {"impl_747"}
// Dependencies: {}
impl NewToken { pub (crate) fn encode < W : BufMut > (& self , out : & mut W) { out . write (FrameType :: NEW_TOKEN) ; out . write_var (self . token . len () as u64) ; out . put_slice (& self . token) ; } pub (crate) fn size (& self) -> usize { 1 + VarInt :: from_u64 (self . token . len () as u64) . unwrap () . size () + self . token . len () } }
};
}
