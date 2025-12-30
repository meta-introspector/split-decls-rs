// Generated macro for impl_357 (impl)
macro_rules! Depcrateimpl_357 {
() => {
// Module: crate
// Provides: {"impl_357"}
// Dependencies: {}
impl HasVisibility for Enum { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
};
}
