// Generated macro for impl_349 (impl)
macro_rules! Depcrateimpl_349 {
() => {
// Module: crate
// Provides: {"impl_349"}
// Dependencies: {}
impl HasVisibility for Struct { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
};
}
