// Generated macro for impl_771 (impl)
macro_rules! Depcrate_future_extimpl_771 {
() => {
// Module: crate::future_ext
// Provides: {"impl_771"}
// Dependencies: {}
impl < T , F > Future for PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { type Output = T ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { (& mut self . f) (cx) } }
};
}
