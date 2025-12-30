// Generated macro for impl_101 (impl)
macro_rules! Depcrate_future_future_fuseimpl_101 {
() => {
// Module: crate::future::future::fuse
// Provides: {"impl_101"}
// Dependencies: {}
impl < Fut : Future > Future for Fuse < Fut > { type Output = Fut :: Output ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Fut :: Output > { match self . as_mut () . project () . inner . as_pin_mut () { Some (fut) => fut . poll (cx) . map (| output | { self . project () . inner . set (None) ; output }) , None => Poll :: Pending , } } }
};
}
