// Generated macro for impl_13 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_13 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_13"}
// Dependencies: {}
impl < T : Schema , const N : usize > Schema for [T ; N] { const SCHEMA : & 'static DataModelType = & DataModelType :: Tuple (& [T :: SCHEMA ; N]) ; }
};
}
