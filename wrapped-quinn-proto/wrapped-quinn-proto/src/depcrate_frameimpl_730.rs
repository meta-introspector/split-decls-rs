// Generated macro for impl_730 (impl)
macro_rules! Depcrate_frameimpl_730 {
() => {
// Module: crate::frame
// Provides: {"impl_730"}
// Dependencies: {}
impl ApplicationClose { pub (crate) fn encode < W : BufMut > (& self , out : & mut W , max_len : usize) { out . write (FrameType :: APPLICATION_CLOSE) ; out . write (self . error_code) ; let max_len = max_len - 3 - VarInt :: from_u64 (self . reason . len () as u64) . unwrap () . size () ; let actual_len = self . reason . len () . min (max_len) ; out . write_var (actual_len as u64) ; out . put_slice (& self . reason [0 .. actual_len]) ; } }
};
}
