// Generated macro for impl_16 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_16 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_16"}
// Dependencies: {}
impl < T : Schema > Schema for RangeFrom < T > { const SCHEMA : & 'static crate :: schema :: NamedType = & NamedType { name : "RangeFrom<T>" , ty : & DataModelType :: Struct (& [& NamedValue { name : "start" , ty : T :: SCHEMA , }]) , } ; }
};
}
