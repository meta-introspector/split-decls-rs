// Generated macro for impl_17 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_17 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_17"}
// Dependencies: {}
impl < T : Schema > Schema for RangeTo < T > { const SCHEMA : & 'static crate :: schema :: NamedType = & NamedType { name : "RangeTo<T>" , ty : & DataModelType :: Struct (& [& NamedValue { name : "end" , ty : T :: SCHEMA , }]) , } ; }
};
}
