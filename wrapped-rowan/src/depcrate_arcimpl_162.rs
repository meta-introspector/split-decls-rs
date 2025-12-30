// Generated macro for impl_162 (impl)
macro_rules! Depcrate_arcimpl_162 {
() => {
// Module: crate::arc
// Provides: {"impl_162"}
// Dependencies: {}
impl < T : ? Sized + Hash > Hash for Arc < T > { fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) } }
};
}
