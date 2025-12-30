// Generated macro for impl_2264 (impl)
macro_rules! Depcrate_io_cursorimpl_2264 {
() => {
// Module: crate::io::cursor
// Provides: {"impl_2264"}
// Dependencies: {}
impl < T > AsyncBufRead for Cursor < T > where T : AsRef < [u8] > + Unpin , { fn poll_fill_buf (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Poll :: Ready (io :: BufRead :: fill_buf (& mut self . get_mut () . inner)) } fn consume (mut self : Pin < & mut Self > , amt : usize) { io :: BufRead :: consume (& mut self . inner , amt) } }
};
}
