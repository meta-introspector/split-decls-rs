// Generated macro for impl_2368 (impl)
macro_rules! Depcrate_io_read_exactimpl_2368 {
() => {
// Module: crate::io::read_exact
// Provides: {"impl_2368"}
// Dependencies: {}
impl < R : AsyncRead + ? Sized + Unpin > Future for ReadExact < '_ , R > { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; while ! this . buf . is_empty () { let n = ready ! (Pin :: new (& mut this . reader) . poll_read (cx , this . buf)) ? ; { let (_ , rest) = mem :: take (& mut this . buf) . split_at_mut (n) ; this . buf = rest ; } if n == 0 { return Poll :: Ready (Err (io :: ErrorKind :: UnexpectedEof . into ())) ; } } Poll :: Ready (Ok (())) } }
};
}
