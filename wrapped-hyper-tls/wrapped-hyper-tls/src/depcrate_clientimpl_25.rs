// Generated macro for impl_25 (impl)
macro_rules! Depcrate_clientimpl_25 {
() => {
// Module: crate::client
// Provides: {"impl_25"}
// Dependencies: {}
impl < T : Read + Write + Unpin > Future for HttpsConnecting < T > { type Output = Result < MaybeHttpsStream < T > , BoxError > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll (cx) } }
};
}
