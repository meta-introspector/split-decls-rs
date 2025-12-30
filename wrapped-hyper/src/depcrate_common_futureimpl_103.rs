// Generated macro for impl_103 (impl)
macro_rules! Depcrate_common_futureimpl_103 {
() => {
// Module: crate::common::future
// Provides: {"impl_103"}
// Dependencies: {}
impl < T , F > Future for PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { type Output = T ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { (self . as_mut () . f) (cx) } }
};
}
