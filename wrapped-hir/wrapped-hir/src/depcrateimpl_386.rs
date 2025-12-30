// Generated macro for impl_386 (impl)
macro_rules! Depcrateimpl_386 {
() => {
// Module: crate
// Provides: {"impl_386"}
// Dependencies: {}
impl HasVisibility for Function { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { db . assoc_visibility (self . id . into ()) } }
};
}
