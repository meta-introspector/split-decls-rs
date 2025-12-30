// Generated macro for impl_177 (impl)
macro_rules! Depcrate_arcimpl_177 {
() => {
// Module: crate::arc
// Provides: {"impl_177"}
// Dependencies: {}
impl < H : Hash , T : Hash > Hash for ThinArc < H , T > { fn hash < HSR : Hasher > (& self , state : & mut HSR) { (* * self) . hash (state) } }
};
}
