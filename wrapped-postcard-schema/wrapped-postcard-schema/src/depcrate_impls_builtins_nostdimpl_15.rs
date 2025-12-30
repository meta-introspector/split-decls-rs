// Generated macro for impl_15 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_15 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_15"}
// Dependencies: {}
impl < T : Schema > Schema for RangeInclusive < T > { const SCHEMA : & 'static crate :: schema :: NamedType = & NamedType { name : "RangeInclusive<T>" , ty : & DataModelType :: Struct (& [& NamedValue { name : "start" , ty : T :: SCHEMA , } , & NamedValue { name : "end" , ty : T :: SCHEMA , } ,]) , } ; }
};
}
