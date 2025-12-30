// Generated macro for impl_95 (impl)
macro_rules! Depcrate_impls_serde_big_array_v0_5impl_95 {
() => {
// Module: crate::impls::serde_big_array_v0_5
// Provides: {"impl_95"}
// Dependencies: {}
impl < T : Schema , const N : usize > Schema for Array < T , N > { const SCHEMA : & 'static NamedType = < [T ; N] as Schema > :: SCHEMA ; }
};
}
