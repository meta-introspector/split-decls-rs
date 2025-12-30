// Generated macro for impl_53 (impl)
macro_rules! Depcrate_implsimpl_53 {
() => {
// Module: crate::impls
// Provides: {"impl_53"}
// Dependencies: {}
impl < T : Hash , N : ArrayLength > Hash for GenericArray < T , N > { # [inline] fn hash < H > (& self , state : & mut H) where H : Hasher , { Hash :: hash (self . as_slice () , state) } }
};
}
