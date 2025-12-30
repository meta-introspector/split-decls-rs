// Generated macro for impl_404 (impl)
macro_rules! Depcrate_future_future_groupimpl_404 {
() => {
// Module: crate::future::future_group
// Provides: {"impl_404"}
// Dependencies: {}
impl < F : Future > Stream for FutureGroup < F > { type Item = < F as Future > :: Output ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match self . poll_next_inner (cx) { Poll :: Ready (Some ((_key , item))) => Poll :: Ready (Some (item)) , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Pending => Poll :: Pending , } } }
};
}
