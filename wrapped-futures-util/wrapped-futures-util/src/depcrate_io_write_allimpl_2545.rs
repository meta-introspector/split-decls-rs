// Generated macro for impl_2545 (impl)
macro_rules! Depcrate_io_write_allimpl_2545 {
() => {
// Module: crate::io::write_all
// Provides: {"impl_2545"}
// Dependencies: {}
impl < W : AsyncWrite + ? Sized + Unpin > Future for WriteAll < '_ , W > { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { let this = & mut * self ; while ! this . buf . is_empty () { let n = ready ! (Pin :: new (& mut this . writer) . poll_write (cx , this . buf)) ? ; { let (_ , rest) = mem :: take (& mut this . buf) . split_at (n) ; this . buf = rest ; } if n == 0 { return Poll :: Ready (Err (io :: ErrorKind :: WriteZero . into ())) ; } } Poll :: Ready (Ok (())) } }
};
}
