// Generated macro for impl_78 (impl)
macro_rules! Depcrate_mockimpl_78 {
() => {
// Module: crate::mock
// Provides: {"impl_78"}
// Dependencies: {}
impl AsyncRead for Pipe { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf ,) -> Poll < io :: Result < () > > { assert ! (buf . remaining () > 0 , "attempted read with zero length buffer... wut?") ; let mut me = self . inner . lock () . unwrap () ; if me . tx . is_empty () { if me . closed { return Poll :: Ready (Ok (())) ; } me . tx_task = Some (cx . waker () . clone ()) ; return Poll :: Pending ; } let n = cmp :: min (buf . remaining () , me . tx . len ()) ; buf . put_slice (& me . tx [.. n]) ; me . tx . drain (.. n) ; Poll :: Ready (Ok (())) } }
};
}
