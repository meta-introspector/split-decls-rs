// Generated macro for impl_324 (impl)
macro_rules! Depcrate_ioimpl_324 {
() => {
// Module: crate::io
// Provides: {"impl_324"}
// Dependencies: {}
impl < R : AsyncRead + Unpin + ? Sized > Future for ReadExactFuture < '_ , R > { type Output = Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , buf } = & mut * self ; while ! buf . is_empty () { let n = ready ! (Pin :: new (& mut * reader) . poll_read (cx , buf)) ? ; let (_ , rest) = mem :: take (buf) . split_at_mut (n) ; * buf = rest ; if n == 0 { return Poll :: Ready (Err (ErrorKind :: UnexpectedEof . into ())) ; } } Poll :: Ready (Ok (())) } }
};
}
