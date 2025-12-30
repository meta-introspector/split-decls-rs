// Generated macro for impl_2262 (impl)
macro_rules! Depcrate_io_cursorimpl_2262 {
() => {
// Module: crate::io::cursor
// Provides: {"impl_2262"}
// Dependencies: {}
impl < T > AsyncSeek for Cursor < T > where T : AsRef < [u8] > + Unpin , { fn poll_seek (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { Poll :: Ready (io :: Seek :: seek (& mut self . inner , pos)) } }
};
}
