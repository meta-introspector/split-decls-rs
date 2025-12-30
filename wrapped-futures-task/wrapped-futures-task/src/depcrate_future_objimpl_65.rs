// Generated macro for impl_65 (impl)
macro_rules! Depcrate_future_objimpl_65 {
() => {
// Module: crate::future_obj
// Provides: {"impl_65"}
// Dependencies: {}
impl < T > Future for FutureObj < '_ , T > { type Output = T ; # [inline] fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { Pin :: new (& mut self . 0) . poll (cx) } }
};
}
