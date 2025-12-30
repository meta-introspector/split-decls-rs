// Generated macro for impl_55 (impl)
macro_rules! Depcrate_serverimpl_55 {
() => {
// Module: crate::server
// Provides: {"impl_55"}
// Dependencies: {}
impl Future for Server { type Output = io :: Result < () > ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut Pin :: into_inner (self) . fut) . poll (cx) } }
};
}
