// Generated macro for impl_14 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_14 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_14"}
// Dependencies: {}
impl < T : Schema > Schema for Range < T > { const SCHEMA : & 'static DataModelType = & DataModelType :: Struct { name : "Range<T>" , data : Data :: Struct (& [& NamedField { name : "start" , ty : T :: SCHEMA , } , & NamedField { name : "end" , ty : T :: SCHEMA , } ,]) , } ; }
};
}
