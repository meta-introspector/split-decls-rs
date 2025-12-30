// Generated macro for impl_76 (impl)
macro_rules! Depcrate_mockimpl_76 {
() => {
// Module: crate::mock
// Provides: {"impl_76"}
// Dependencies: {}
impl AsyncWrite for Mock { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , mut buf : & [u8] ,) -> Poll < Result < usize , io :: Error > > { let mut me = self . pipe . inner . lock () . unwrap () ; if me . closed { return Poll :: Ready (Ok (buf . len ())) ; } if me . tx_rem == 0 { me . tx_rem_task = Some (cx . waker () . clone ()) ; return Poll :: Pending ; } if buf . len () > me . tx_rem { buf = & buf [.. me . tx_rem] ; } me . tx . extend (buf) ; me . tx_rem -= buf . len () ; if let Some (task) = me . tx_task . take () { task . wake () ; } Poll :: Ready (Ok (buf . len ())) } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , io :: Error > > { Poll :: Ready (Ok (())) } fn poll_shutdown (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , io :: Error > > { Poll :: Ready (Ok (())) } }
};
}
