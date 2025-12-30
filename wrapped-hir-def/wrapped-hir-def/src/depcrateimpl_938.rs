// Generated macro for impl_938 (impl)
macro_rules! Depcrateimpl_938 {
() => {
// Module: crate
// Provides: {"impl_938"}
// Dependencies: {}
impl HasModule for VariantId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match * self { VariantId :: EnumVariantId (it) => it . module (db) , VariantId :: StructId (it) => it . module (db) , VariantId :: UnionId (it) => it . module (db) , } } }
};
}
