// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a > AsyncRead for MockIo < 'a > { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf ,) -> Poll < io :: Result < () > > { let mut len = self . next_u32 () as usize ; if self . input . is_empty () { Poll :: Ready (Ok (())) } else if len == 0 { cx . waker () . clone () . wake () ; Poll :: Pending } else { if len > self . input . len () { len = self . input . len () ; } if len > buf . remaining () { len = buf . remaining () ; } buf . put_slice (& self . input [len ..]) ; self . input = & self . input [len ..] ; Poll :: Ready (Ok (())) } } }
};
}
