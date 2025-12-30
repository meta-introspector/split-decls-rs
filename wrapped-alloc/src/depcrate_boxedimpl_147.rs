// Generated macro for impl_147 (impl)
macro_rules! Depcrate_boxedimpl_147 {
() => {
// Module: crate::boxed
// Provides: {"impl_147"}
// Dependencies: {}
# [stable (feature = "async_closure" , since = "1.85.0")] impl < Args : Tuple , F : AsyncFn < Args > + ? Sized , A : Allocator > AsyncFn < Args > for Box < F , A > { extern "rust-call" fn async_call (& self , args : Args) -> Self :: CallRefFuture < '_ > { F :: async_call (self , args) } }
};
}
