// Generated macro for impl_142 (impl)
macro_rules! Depcrate_boxedimpl_142 {
() => {
// Module: crate::boxed
// Provides: {"impl_142"}
// Dependencies: {}
# [stable (feature = "boxed_closure_impls" , since = "1.35.0")] impl < Args : Tuple , F : FnOnce < Args > + ? Sized , A : Allocator > FnOnce < Args > for Box < F , A > { type Output = < F as FnOnce < Args > > :: Output ; extern "rust-call" fn call_once (self , args : Args) -> Self :: Output { < F as FnOnce < Args > > :: call_once (* self , args) } }
};
}
