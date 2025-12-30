// Generated macro for impl_503 (impl)
macro_rules! Depcrate_rt_tokioimpl_503 {
() => {
// Module: crate::rt::tokio
// Provides: {"impl_503"}
// Dependencies: {}
impl Future for TokioSleep { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . inner . poll (cx) } }
};
}
