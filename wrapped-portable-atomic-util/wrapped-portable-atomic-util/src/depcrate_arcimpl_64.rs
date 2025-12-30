// Generated macro for impl_64 (impl)
macro_rules! Depcrate_arcimpl_64 {
() => {
// Module: crate::arc
// Provides: {"impl_64"}
// Dependencies: {}
impl < T : ? Sized + Hash > Hash for Arc < T > { fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) ; } }
};
}
