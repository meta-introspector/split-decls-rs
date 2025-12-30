// Generated macro for impl_181 (impl)
macro_rules! Depcrate_context_itemsimpl_181 {
() => {
// Module: crate::context_items
// Provides: {"impl_181"}
// Dependencies: {}
impl < A , M > ActorFuture < A > for ActorDelayedMessageItem < M > where A : Actor + Handler < M > , A :: Context : AsyncContext < A > , M : Message + 'static , { type Output = () ; fn poll (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut task :: Context < '_ > ,) -> Poll < Self :: Output > { let this = self . project () ; ready ! (this . timeout . poll (task)) ; let fut = A :: handle (act , this . msg . take () . unwrap () , ctx) ; fut . handle (ctx , None) ; Poll :: Ready (()) } }
};
}
