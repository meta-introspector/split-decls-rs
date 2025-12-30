// Generated macro for impl_487 (impl)
macro_rules! Depcrate_ioimpl_487 {
() => {
// Module: crate::io
// Provides: {"impl_487"}
// Dependencies: {}
impl < I : 'static , S : Sink < I > , A > ActorFuture < A > for SinkWriteFuture < I , S > where S : Sink < I > + Unpin , A : Actor + WriteHandler < S :: Error > , A :: Context : AsyncContext < A > , { type Output = () ; fn poll (self : Pin < & mut Self > , act : & mut A , ctxt : & mut A :: Context , cx : & mut Context < '_ > ,) -> Poll < Self :: Output > { let this = self . get_mut () ; let inner = & mut this . inner . borrow_mut () ; loop { match Pin :: new (& mut inner . sink) . poll_ready (cx) { Poll :: Ready (Ok (())) => { if let Some (item) = inner . buffer . pop_front () { let _ = Pin :: new (& mut inner . sink) . start_send (item) ; } else { break ; } } Poll :: Ready (Err (_err)) => { break ; } Poll :: Pending => { break ; } } } if ! inner . closing_flag . contains (Flags :: CLOSING) { match Pin :: new (& mut inner . sink) . poll_flush (cx) { Poll :: Ready (Err (e)) => { if act . error (e , ctxt) == Running :: Stop { act . finished (ctxt) ; return Poll :: Ready (()) ; } } Poll :: Ready (Ok (())) => { } Poll :: Pending => { } } } else { assert ! (! inner . closing_flag . contains (Flags :: CLOSED)) ; match Pin :: new (& mut inner . sink) . poll_close (cx) { Poll :: Ready (Err (e)) => { if act . error (e , ctxt) == Running :: Stop { act . finished (ctxt) ; return Poll :: Ready (()) ; } } Poll :: Ready (Ok (())) => { if inner . buffer . is_empty () { inner . closing_flag |= Flags :: CLOSED ; act . finished (ctxt) ; return Poll :: Ready (()) ; } } Poll :: Pending => { } } } inner . task . replace (cx . waker () . clone ()) ; Poll :: Pending } }
};
}
