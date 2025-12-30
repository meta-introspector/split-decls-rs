// Generated macro for impl_335 (impl)
macro_rules! Depcrateimpl_335 {
() => {
// Module: crate
// Provides: {"impl_335"}
// Dependencies: {}
impl HasVisibility for Module { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let def_map = self . id . def_map (db) ; let module_data = & def_map [self . id . local_id] ; module_data . visibility } }
};
}
