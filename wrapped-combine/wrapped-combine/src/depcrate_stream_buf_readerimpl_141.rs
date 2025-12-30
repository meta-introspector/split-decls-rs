// Generated macro for impl_141 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_141 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_141"}
// Dependencies: {}
impl < R : Read > BufRead for BufReader < R > { fn fill_buf (& mut self) -> io :: Result < & [u8] > { if self . buf . is_empty () { Bufferless . extend_buf_sync (self) ? ; } Ok (& self . buf [..]) } fn consume (& mut self , amt : usize) { self . buf . advance (amt) ; } }
};
}
