// Generated macro for impl_188 (impl)
macro_rules! Depcrate_context_itemsimpl_188 {
() => {
// Module: crate::context_items
// Provides: {"impl_188"}
// Dependencies: {}
impl < A , S > ActorFuture < A > for ActorMessageStreamItem < S > where S : Stream , A : Actor + Handler < S :: Item > , A :: Context : AsyncContext < A > , S :: Item : Message + 'static , { type Output = () ; fn poll (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut task :: Context < '_ > ,) -> Poll < Self :: Output > { let mut this = self . project () ; while let Some (msg) = ready ! (this . stream . as_mut () . poll_next (task)) { let fut = Handler :: handle (act , msg , ctx) ; fut . handle (ctx , None) ; if ctx . waiting () { return Poll :: Pending ; } } Poll :: Ready (()) } }
};
}
