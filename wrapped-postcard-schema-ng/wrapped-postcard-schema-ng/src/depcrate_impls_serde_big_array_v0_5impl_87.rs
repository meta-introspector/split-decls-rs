// Generated macro for impl_87 (impl)
macro_rules! Depcrate_impls_serde_big_array_v0_5impl_87 {
() => {
// Module: crate::impls::serde_big_array_v0_5
// Provides: {"impl_87"}
// Dependencies: {}
impl < T : Schema , const N : usize > Schema for Array < T , N > { const SCHEMA : & 'static DataModelType = < [T ; N] as Schema > :: SCHEMA ; }
};
}
