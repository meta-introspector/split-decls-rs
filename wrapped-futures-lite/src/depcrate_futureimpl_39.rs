// Generated macro for impl_39 (impl)
macro_rules! Depcrate_futureimpl_39 {
() => {
// Module: crate::future
// Provides: {"impl_39"}
// Dependencies: {}
impl < Fut : Future > Future for Fuse < Fut > { type Output = Fut :: Output ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Fut :: Output > { match self . as_mut () . project () . inner . as_pin_mut () . map (| f | f . poll (cx)) { Some (Poll :: Ready (output)) => { self . project () . inner . set (None) ; Poll :: Ready (output) } Some (Poll :: Pending) | None => Poll :: Pending , } } }
};
}
