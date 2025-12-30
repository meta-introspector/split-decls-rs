// Generated macro for impl_647 (impl)
macro_rules! Depcrate_traitsimpl_647 {
() => {
// Module: crate::traits
// Provides: {"impl_647"}
// Dependencies: {}
impl < T : Hash > Hash for Obligation < '_ , T > { fn hash < H : Hasher > (& self , state : & mut H) -> () { self . param_env . hash (state) ; self . predicate . hash (state) ; } }
};
}
