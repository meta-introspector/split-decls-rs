// Generated macro for impl_275 (impl)
macro_rules! Depcrate_ioimpl_275 {
() => {
// Module: crate::io
// Provides: {"impl_275"}
// Dependencies: {}
impl < T > AsyncBufRead for Cursor < T > where T : AsRef < [u8] > + Unpin , { fn poll_fill_buf (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < & [u8] > > { Poll :: Ready (std :: io :: BufRead :: fill_buf (& mut self . get_mut () . inner)) } fn consume (mut self : Pin < & mut Self > , amt : usize) { std :: io :: BufRead :: consume (& mut self . inner , amt) } }
};
}
