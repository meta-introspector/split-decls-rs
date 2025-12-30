// Generated macro for impl_259 (impl)
macro_rules! Depcrate_streamimpl_259 {
() => {
// Module: crate::stream
// Provides: {"impl_259"}
// Dependencies: {}
impl < A , S > ActorFuture < A > for ActorStream < S > where S : Stream , A : Actor + StreamHandler < S :: Item > , A :: Context : AsyncContext < A > , { type Output = () ; fn poll (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { let mut this = self . project () ; if ! * this . started { * this . started = true ; < A as StreamHandler < S :: Item > > :: started (act , ctx) ; } let mut polled = 0 ; while let Some (msg) = ready ! (this . stream . as_mut () . poll_next (task)) { A :: handle (act , msg , ctx) ; polled += 1 ; if ctx . waiting () { return Poll :: Pending ; } else if polled == 16 { task . waker () . wake_by_ref () ; return Poll :: Pending ; } } A :: finished (act , ctx) ; Poll :: Ready (()) } }
};
}
