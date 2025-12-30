// Generated macro for impl_402 (impl)
macro_rules! Depcrateimpl_402 {
() => {
// Module: crate
// Provides: {"impl_402"}
// Dependencies: {}
impl TypeAlias { pub fn has_non_default_type_params (self , db : & dyn HirDatabase) -> bool { has_non_default_type_params (db , self . id . into ()) } pub fn module (self , db : & dyn HirDatabase) -> Module { Module { id : self . id . module (db) } } pub fn ty (self , db : & dyn HirDatabase) -> Type < '_ > { Type :: from_def (db , self . id) } pub fn ty_params (self , db : & dyn HirDatabase) -> Type < '_ > { Type :: from_def_params (db , self . id) } pub fn name (self , db : & dyn HirDatabase) -> Name { db . type_alias_signature (self . id) . name . clone () } }
};
}
