// Generated macro for impl_346 (impl)
macro_rules! Depcrateimpl_346 {
() => {
// Module: crate
// Provides: {"impl_346"}
// Dependencies: {}
impl HasVisibility for Field { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let variant_data = VariantId :: from (self . parent) . fields (db) ; let visibility = & variant_data . fields () [self . id] . visibility ; let parent_id : hir_def :: VariantId = self . parent . into () ; Visibility :: resolve (db , & parent_id . resolver (db) , visibility) } }
};
}
