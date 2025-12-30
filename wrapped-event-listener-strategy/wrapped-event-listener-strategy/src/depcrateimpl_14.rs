// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < F : EventListenerFuture + ? Sized > Future for FutureWrapper < F > { type Output = F :: Output ; # [inline] fn poll (self : Pin < & mut Self > , context : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . inner . poll_with_strategy (& mut NonBlocking :: default () , context) } }
};
}
