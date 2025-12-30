// Generated macro for impl_366 (impl)
macro_rules! Depcrateimpl_366 {
() => {
// Module: crate
// Provides: {"impl_366"}
// Dependencies: {}
# [doc = " Variants inherit visibility from the parent enum."] impl HasVisibility for Variant { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { self . parent_enum (db) . visibility (db) } }
};
}
