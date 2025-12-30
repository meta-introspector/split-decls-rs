// Generated macro for impl_393 (impl)
macro_rules! Depcrateimpl_393 {
() => {
// Module: crate
// Provides: {"impl_393"}
// Dependencies: {}
impl HasVisibility for TraitAlias { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let loc = self . id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , self . id , source . map (| src | src . visibility ())) } }
};
}
