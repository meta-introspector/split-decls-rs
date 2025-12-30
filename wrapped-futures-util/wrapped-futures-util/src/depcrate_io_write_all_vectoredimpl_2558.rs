// Generated macro for impl_2558 (impl)
macro_rules! Depcrate_io_write_all_vectoredimpl_2558 {
() => {
// Module: crate::io::write_all_vectored
// Provides: {"impl_2558"}
// Dependencies: {}
impl < W : AsyncWrite + ? Sized + Unpin > Future for WriteAllVectored < '_ , '_ , W > { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { let this = & mut * self ; while ! this . bufs . is_empty () { let n = ready ! (Pin :: new (& mut this . writer) . poll_write_vectored (cx , this . bufs)) ? ; if n == 0 { return Poll :: Ready (Err (io :: ErrorKind :: WriteZero . into ())) ; } else { IoSlice :: advance_slices (& mut this . bufs , n) ; } } Poll :: Ready (Ok (())) } }
};
}
