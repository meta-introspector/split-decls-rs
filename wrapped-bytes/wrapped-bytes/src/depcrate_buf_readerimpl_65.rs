// Generated macro for impl_65 (impl)
macro_rules! Depcrate_buf_readerimpl_65 {
() => {
// Module: crate::buf::reader
// Provides: {"impl_65"}
// Dependencies: {}
impl < B : Buf + Sized > io :: BufRead for Reader < B > { fn fill_buf (& mut self) -> io :: Result < & [u8] > { Ok (self . buf . chunk ()) } fn consume (& mut self , amt : usize) { self . buf . advance (amt) } }
};
}
