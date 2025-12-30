// Generated macro for impl_178 (impl)
macro_rules! Depcrate_context_itemsimpl_178 {
() => {
// Module: crate::context_items
// Provides: {"impl_178"}
// Dependencies: {}
impl < A > ActorWaitItem < A > where A : Actor , A :: Context : ActorContext + AsyncContext < A > , { # [inline] pub fn new < F > (fut : F) -> Self where F : ActorFuture < A , Output = () > + 'static , { ActorWaitItem (Box :: pin (fut)) } pub fn poll (mut self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut task :: Context < '_ > ,) -> Poll < () > { match self . 0 . as_mut () . poll (act , ctx , task) { Poll :: Pending => { if ctx . state () . alive () { Poll :: Pending } else { Poll :: Ready (()) } } Poll :: Ready (_) => Poll :: Ready (()) , } } }
};
}
