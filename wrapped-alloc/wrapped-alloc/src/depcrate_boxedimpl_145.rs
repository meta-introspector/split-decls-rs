// Generated macro for impl_145 (impl)
macro_rules! Depcrate_boxedimpl_145 {
() => {
// Module: crate::boxed
// Provides: {"impl_145"}
// Dependencies: {}
# [stable (feature = "async_closure" , since = "1.85.0")] impl < Args : Tuple , F : AsyncFnOnce < Args > + ? Sized , A : Allocator > AsyncFnOnce < Args > for Box < F , A > { type Output = F :: Output ; type CallOnceFuture = F :: CallOnceFuture ; extern "rust-call" fn async_call_once (self , args : Args) -> Self :: CallOnceFuture { F :: async_call_once (* self , args) } }
};
}
