// Generated macro for impl_22 (impl)
macro_rules! Depcrate_futureimpl_22 {
() => {
// Module: crate::future
// Provides: {"impl_22"}
// Dependencies: {}
impl < T , F > Future for PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { type Output = T ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { let this = self . project () ; (this . f) (cx) } }
};
}
