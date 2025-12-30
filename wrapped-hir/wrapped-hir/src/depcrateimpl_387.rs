// Generated macro for impl_387 (impl)
macro_rules! Depcrateimpl_387 {
() => {
// Module: crate
// Provides: {"impl_387"}
// Dependencies: {}
impl HasVisibility for Static { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
};
}
