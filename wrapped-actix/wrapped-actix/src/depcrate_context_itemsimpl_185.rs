// Generated macro for impl_185 (impl)
macro_rules! Depcrate_context_itemsimpl_185 {
() => {
// Module: crate::context_items
// Provides: {"impl_185"}
// Dependencies: {}
impl < A , M > ActorFuture < A > for ActorMessageItem < M > where A : Actor + Handler < M > , A :: Context : AsyncContext < A > , M : Message + 'static , { type Output = () ; fn poll (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , _ : & mut task :: Context < '_ > ,) -> Poll < Self :: Output > { let this = self . get_mut () ; let fut = Handler :: handle (act , this . msg . take () . unwrap () , ctx) ; fut . handle (ctx , None) ; Poll :: Ready (()) } }
};
}
