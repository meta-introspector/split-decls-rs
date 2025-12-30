// Generated macro for impl_370 (impl)
macro_rules! Depcrateimpl_370 {
() => {
// Module: crate
// Provides: {"impl_370"}
// Dependencies: {}
impl HasVisibility for Adt { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { match self { Adt :: Struct (it) => it . visibility (db) , Adt :: Union (it) => it . visibility (db) , Adt :: Enum (it) => it . visibility (db) , } } }
};
}
