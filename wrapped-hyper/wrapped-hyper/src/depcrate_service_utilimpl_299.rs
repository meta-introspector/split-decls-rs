// Generated macro for impl_299 (impl)
macro_rules! Depcrate_service_utilimpl_299 {
() => {
// Module: crate::service::util
// Provides: {"impl_299"}
// Dependencies: {}
impl < F , R > Clone for ServiceFn < F , R > where F : Clone , { fn clone (& self) -> Self { ServiceFn { f : self . f . clone () , _req : PhantomData , } } }
};
}
