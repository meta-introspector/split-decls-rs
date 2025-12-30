// Generated macro for impl_58 (impl)
macro_rules! Depcrate_future_objimpl_58 {
() => {
// Module: crate::future_obj
// Provides: {"impl_58"}
// Dependencies: {}
impl < T > Future for LocalFutureObj < '_ , T > { type Output = T ; # [inline] fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { unsafe { Pin :: new_unchecked (& mut * self . future) . poll (cx) } } }
};
}
