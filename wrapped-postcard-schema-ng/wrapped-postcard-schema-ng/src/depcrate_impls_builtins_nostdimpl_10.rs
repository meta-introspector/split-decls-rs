// Generated macro for impl_10 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_10 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_10"}
// Dependencies: {}
impl < T : Schema , E : Schema > Schema for Result < T , E > { const SCHEMA : & 'static DataModelType = & DataModelType :: Enum { name : "Result<T, E>" , variants : & [& Variant { name : "Ok" , data : Data :: Newtype (T :: SCHEMA) , } , & Variant { name : "Err" , data : Data :: Newtype (E :: SCHEMA) , } ,] , } ; }
};
}
