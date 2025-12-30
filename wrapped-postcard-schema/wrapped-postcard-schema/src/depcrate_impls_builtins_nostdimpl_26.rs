// Generated macro for impl_26 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_26 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_26"}
// Dependencies: {}
impl Schema for Duration { const SCHEMA : & 'static NamedType = & NamedType { name : "Duration" , ty : & DataModelType :: Struct (& [& NamedValue { name : "secs" , ty : u64 :: SCHEMA , } , & NamedValue { name : "nanos" , ty : u32 :: SCHEMA , } ,]) , } ; }
};
}
