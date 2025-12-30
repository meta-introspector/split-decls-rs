// Generated macro for impl_354 (impl)
macro_rules! Depcrateimpl_354 {
() => {
// Module: crate
// Provides: {"impl_354"}
// Dependencies: {}
impl HasVisibility for Union { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
};
}
