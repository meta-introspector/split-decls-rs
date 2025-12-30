// Generated macro for impl_59 (impl)
macro_rules! Depcrate_future_objimpl_59 {
() => {
// Module: crate::future_obj
// Provides: {"impl_59"}
// Dependencies: {}
impl < T > Drop for LocalFutureObj < '_ , T > { fn drop (& mut self) { unsafe { (self . drop_fn) (self . future) } } }
};
}
