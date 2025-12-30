// Generated macro for impl_247 (impl)
macro_rules! Depcrate_ioimpl_247 {
() => {
// Module: crate::io
// Provides: {"impl_247"}
// Dependencies: {}
impl < T : AsyncSeek + Unpin > std :: io :: Seek for AsyncAsSync < '_ , '_ , T > { # [inline] fn seek (& mut self , pos : SeekFrom) -> Result < u64 > { self . poll_with (| io , cx | io . poll_seek (cx , pos)) } }
};
}
