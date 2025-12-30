// Generated macro for impl_79 (impl)
macro_rules! Depcrate_mockimpl_79 {
() => {
// Module: crate::mock
// Provides: {"impl_79"}
// Dependencies: {}
impl AsyncWrite for Pipe { fn poll_write (self : Pin < & mut Self > , _cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize , io :: Error > > { let mut me = self . inner . lock () . unwrap () ; me . rx . extend (buf) ; if let Some (task) = me . rx_task . take () { task . wake () ; } Poll :: Ready (Ok (buf . len ())) } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , io :: Error > > { Poll :: Ready (Ok (())) } fn poll_shutdown (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , io :: Error > > { Poll :: Ready (Ok (())) } }
};
}
