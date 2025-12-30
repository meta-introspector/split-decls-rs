// Generated macro for impl_144 (impl)
macro_rules! Depcrate_boxedimpl_144 {
() => {
// Module: crate::boxed
// Provides: {"impl_144"}
// Dependencies: {}
# [stable (feature = "boxed_closure_impls" , since = "1.35.0")] impl < Args : Tuple , F : Fn < Args > + ? Sized , A : Allocator > Fn < Args > for Box < F , A > { extern "rust-call" fn call (& self , args : Args) -> Self :: Output { < F as Fn < Args > > :: call (self , args) } }
};
}
