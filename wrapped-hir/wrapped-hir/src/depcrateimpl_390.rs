// Generated macro for impl_390 (impl)
macro_rules! Depcrateimpl_390 {
() => {
// Module: crate
// Provides: {"impl_390"}
// Dependencies: {}
impl HasVisibility for Trait { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
};
}
