// Generated macro for impl_273 (impl)
macro_rules! Depcrate_ioimpl_273 {
() => {
// Module: crate::io
// Provides: {"impl_273"}
// Dependencies: {}
impl < T > AsyncSeek for Cursor < T > where T : AsRef < [u8] > + Unpin , { fn poll_seek (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < Result < u64 > > { Poll :: Ready (std :: io :: Seek :: seek (& mut self . inner , pos)) } }
};
}
