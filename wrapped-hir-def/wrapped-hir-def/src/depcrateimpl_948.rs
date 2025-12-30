// Generated macro for impl_948 (impl)
macro_rules! Depcrateimpl_948 {
() => {
// Module: crate
// Provides: {"impl_948"}
// Dependencies: {}
impl HasModule for GenericDefId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match self { GenericDefId :: FunctionId (it) => it . module (db) , GenericDefId :: AdtId (it) => it . module (db) , GenericDefId :: TraitId (it) => it . module (db) , GenericDefId :: TraitAliasId (it) => it . module (db) , GenericDefId :: TypeAliasId (it) => it . module (db) , GenericDefId :: ImplId (it) => it . module (db) , GenericDefId :: ConstId (it) => it . module (db) , GenericDefId :: StaticId (it) => it . module (db) , } } }
};
}
