// Generated macro for impl_397 (impl)
macro_rules! Depcrateimpl_397 {
() => {
// Module: crate
// Provides: {"impl_397"}
// Dependencies: {}
impl HasVisibility for Static { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
};
}
