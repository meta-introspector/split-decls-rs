// Generated macro for impl_910 (impl)
macro_rules! Depcrateimpl_910 {
() => {
// Module: crate
// Provides: {"impl_910"}
// Dependencies: {}
impl < T , F : FnMut (& mut Context < '_ >) -> Poll < T > > Future for PollFn < F > { type Output = T ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { (self . 0) (cx) } }
};
}
