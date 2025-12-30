// Generated macro for impl_73 (impl)
macro_rules! Depcrate_boxedimpl_73 {
() => {
// Module: crate::boxed
// Provides: {"impl_73"}
// Dependencies: {}
impl < T : ? Sized + Hash , A : Allocator > Hash for Box < T , A > { # [inline (always)] fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) ; } }
};
}
