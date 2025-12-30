// Generated macro for impl_944 (impl)
macro_rules! Depcrateimpl_944 {
() => {
// Module: crate
// Provides: {"impl_944"}
// Dependencies: {}
impl HasModule for AdtId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match * self { AdtId :: StructId (it) => it . module (db) , AdtId :: UnionId (it) => it . module (db) , AdtId :: EnumId (it) => it . module (db) , } } }
};
}
