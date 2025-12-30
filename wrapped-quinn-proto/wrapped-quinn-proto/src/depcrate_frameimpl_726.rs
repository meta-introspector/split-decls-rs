// Generated macro for impl_726 (impl)
macro_rules! Depcrate_frameimpl_726 {
() => {
// Module: crate::frame
// Provides: {"impl_726"}
// Dependencies: {}
impl ConnectionClose { pub (crate) fn encode < W : BufMut > (& self , out : & mut W , max_len : usize) { out . write (FrameType :: CONNECTION_CLOSE) ; out . write (self . error_code) ; let ty = self . frame_type . map_or (0 , | x | x . 0) ; out . write_var (ty) ; let max_len = max_len - 3 - VarInt :: from_u64 (ty) . unwrap () . size () - VarInt :: from_u64 (self . reason . len () as u64) . unwrap () . size () ; let actual_len = self . reason . len () . min (max_len) ; out . write_var (actual_len as u64) ; out . put_slice (& self . reason [0 .. actual_len]) ; } }
};
}
