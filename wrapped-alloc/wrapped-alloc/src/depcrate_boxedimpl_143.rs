// Generated macro for impl_143 (impl)
macro_rules! Depcrate_boxedimpl_143 {
() => {
// Module: crate::boxed
// Provides: {"impl_143"}
// Dependencies: {}
# [stable (feature = "boxed_closure_impls" , since = "1.35.0")] impl < Args : Tuple , F : FnMut < Args > + ? Sized , A : Allocator > FnMut < Args > for Box < F , A > { extern "rust-call" fn call_mut (& mut self , args : Args) -> Self :: Output { < F as FnMut < Args > > :: call_mut (self , args) } }
};
}
