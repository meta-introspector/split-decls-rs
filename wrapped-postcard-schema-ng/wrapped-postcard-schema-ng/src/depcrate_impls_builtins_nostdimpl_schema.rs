// Generated macro for impl_schema (macro)
macro_rules! Depcrate_impls_builtins_nostdimpl_schema {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_schema"}
// Dependencies: {}
macro_rules ! impl_schema { ($ ($ t : ty : $ sdm : expr) ,*) => { $ (impl Schema for $ t { const SCHEMA : &'static DataModelType = &$ sdm ; }) * } ; (tuple => [$ (($ ($ generic : ident) ,*)) ,*]) => { $ (impl <$ ($ generic : Schema) ,*> Schema for ($ ($ generic ,) *) { const SCHEMA : &'static DataModelType = & DataModelType :: Tuple (& [$ ($ generic :: SCHEMA) ,*]) ; }) * } ; }
};
}
