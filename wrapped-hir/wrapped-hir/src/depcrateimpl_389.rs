// Generated macro for impl_389 (impl)
macro_rules! Depcrateimpl_389 {
() => {
// Module: crate
// Provides: {"impl_389"}
// Dependencies: {}
impl HasVisibility for ExternCrateDecl { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
};
}
