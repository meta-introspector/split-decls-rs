// Generated macro for impl_14 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_14 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_14"}
// Dependencies: {}
impl < T : Schema > Schema for Range < T > { const SCHEMA : & 'static crate :: schema :: NamedType = & NamedType { name : "Range<T>" , ty : & DataModelType :: Struct (& [& NamedValue { name : "start" , ty : T :: SCHEMA , } , & NamedValue { name : "end" , ty : T :: SCHEMA , } ,]) , } ; }
};
}
