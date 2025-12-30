// Generated macro for impl_14 (impl)
macro_rules! Depcrate_boxedimpl_14 {
() => {
// Module: crate::boxed
// Provides: {"impl_14"}
// Dependencies: {}
impl < 'a , T : ? Sized + Hash > Hash for Box < 'a , T > { fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) ; } }
};
}
