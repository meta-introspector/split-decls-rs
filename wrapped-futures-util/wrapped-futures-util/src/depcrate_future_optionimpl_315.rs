// Generated macro for impl_315 (impl)
macro_rules! Depcrate_future_optionimpl_315 {
() => {
// Module: crate::future::option
// Provides: {"impl_315"}
// Dependencies: {}
impl < F : Future > Future for OptionFuture < F > { type Output = Option < F :: Output > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () . inner . as_pin_mut () { Some (x) => x . poll (cx) . map (Some) , None => Poll :: Ready (None) , } } }
};
}
