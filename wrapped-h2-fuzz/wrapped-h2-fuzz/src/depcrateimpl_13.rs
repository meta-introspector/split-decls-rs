// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a > AsyncWrite for MockIo < 'a > { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { let len = std :: cmp :: min (self . next_u32 () as usize , buf . len ()) ; if len == 0 { if self . input . is_empty () { Poll :: Ready (Err (io :: ErrorKind :: BrokenPipe . into ())) } else { cx . waker () . clone () . wake () ; Poll :: Pending } } else { Poll :: Ready (Ok (len)) } } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } fn poll_shutdown (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
