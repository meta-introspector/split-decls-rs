// Generated macro for impl_140 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_140 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_140"}
// Dependencies: {}
impl < R : Read > Read for BufReader < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if ! self . buf . has_remaining_mut () && buf . len () >= self . buf . len () { let res = self . read (buf) ; self . buf . clear () ; return res ; } let nread = { let mut rem = self . fill_buf () ? ; rem . read (buf) ? } ; self . consume (nread) ; Ok (nread) } }
};
}
