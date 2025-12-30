// Generated macro for impl_10 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_10 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_10"}
// Dependencies: {}
impl < T : Schema , E : Schema > Schema for Result < T , E > { const SCHEMA : & 'static NamedType = & NamedType { name : "Result<T, E>" , ty : & DataModelType :: Enum (& [& NamedVariant { name : "Ok" , ty : & DataModelVariant :: TupleVariant (& [T :: SCHEMA]) , } , & NamedVariant { name : "Err" , ty : & DataModelVariant :: TupleVariant (& [E :: SCHEMA]) , } ,]) , } ; }
};
}
