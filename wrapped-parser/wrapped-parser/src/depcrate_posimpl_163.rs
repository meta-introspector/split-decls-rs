// Generated macro for impl_163 (impl)
macro_rules! Depcrate_posimpl_163 {
() => {
// Module: crate::pos
// Provides: {"impl_163"}
// Dependencies: {}
impl < T : Hash > Hash for Positioned < T > { fn hash < H : Hasher > (& self , state : & mut H) { self . node . hash (state) } }
};
}
