// Generated macro for impl_337 (impl)
macro_rules! Depcrateimpl_337 {
() => {
// Module: crate
// Provides: {"impl_337"}
// Dependencies: {}
impl HasVisibility for Module { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let def_map = self . id . def_map (db) ; let module_data = & def_map [self . id . local_id] ; module_data . visibility } }
};
}
