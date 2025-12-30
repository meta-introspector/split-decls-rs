// Generated macro for impl_15 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_15 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_15"}
// Dependencies: {}
impl < T : Schema > Schema for RangeInclusive < T > { const SCHEMA : & 'static DataModelType = & DataModelType :: Struct { name : "RangeInclusive<T>" , data : Data :: Struct (& [& NamedField { name : "start" , ty : T :: SCHEMA , } , & NamedField { name : "end" , ty : T :: SCHEMA , } ,]) , } ; }
};
}
