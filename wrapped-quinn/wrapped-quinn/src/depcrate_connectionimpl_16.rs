// Generated macro for impl_16 (impl)
macro_rules! Depcrate_connectionimpl_16 {
() => {
// Module: crate::connection
// Provides: {"impl_16"}
// Dependencies: {}
impl Future for ZeroRttAccepted { type Output = bool ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll (cx) . map (| x | x . unwrap_or (false)) } }
};
}
