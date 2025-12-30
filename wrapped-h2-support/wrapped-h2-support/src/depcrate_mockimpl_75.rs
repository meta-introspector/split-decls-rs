// Generated macro for impl_75 (impl)
macro_rules! Depcrate_mockimpl_75 {
() => {
// Module: crate::mock
// Provides: {"impl_75"}
// Dependencies: {}
impl AsyncRead for Mock { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf ,) -> Poll < io :: Result < () > > { assert ! (buf . remaining () > 0 , "attempted read with zero length buffer... wut?") ; let mut me = self . pipe . inner . lock () . unwrap () ; if me . unexpected_eof { return Poll :: Ready (Err (io :: Error :: new (io :: ErrorKind :: UnexpectedEof , "Simulate an unexpected eof error" ,))) ; } if me . rx . is_empty () { if me . closed { return Poll :: Ready (Ok (())) ; } me . rx_task = Some (cx . waker () . clone ()) ; return Poll :: Pending ; } let n = cmp :: min (buf . remaining () , me . rx . len ()) ; buf . put_slice (& me . rx [.. n]) ; me . rx . drain (.. n) ; Poll :: Ready (Ok (())) } }
};
}
