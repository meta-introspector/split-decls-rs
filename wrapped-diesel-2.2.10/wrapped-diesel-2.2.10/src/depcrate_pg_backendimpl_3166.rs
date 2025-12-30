// Generated macro for impl_3166 (impl)
macro_rules! Depcrate_pg_backendimpl_3166 {
() => {
// Module: crate::pg::backend
// Provides: {"impl_3166"}
// Dependencies: {}
impl std :: fmt :: Display for FailedToLookupTypeError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if let Some (schema) = self . 0 . schema . as_ref () { write ! (f , "Failed to find a type oid for `{}`.`{}`" , schema , self . 0 . type_name) } else { write ! (f , "Failed to find a type oid for `{}`" , self . 0 . type_name) } } }
};
}
