// Generated macro for impl_146 (impl)
macro_rules! Depcrate_boxedimpl_146 {
() => {
// Module: crate::boxed
// Provides: {"impl_146"}
// Dependencies: {}
# [stable (feature = "async_closure" , since = "1.85.0")] impl < Args : Tuple , F : AsyncFnMut < Args > + ? Sized , A : Allocator > AsyncFnMut < Args > for Box < F , A > { type CallRefFuture < 'a > = F :: CallRefFuture < 'a > where Self : 'a ; extern "rust-call" fn async_call_mut (& mut self , args : Args) -> Self :: CallRefFuture < '_ > { F :: async_call_mut (self , args) } }
};
}
