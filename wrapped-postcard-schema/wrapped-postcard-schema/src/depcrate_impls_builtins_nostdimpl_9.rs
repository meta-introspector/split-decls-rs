// Generated macro for impl_9 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_9 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_9"}
// Dependencies: {}
impl < T : Schema > Schema for Option < T > { const SCHEMA : & 'static NamedType = & NamedType { name : "Option<T>" , ty : & DataModelType :: Option (T :: SCHEMA) , } ; }
};
}
