// Generated macro for impl_452 (impl)
macro_rules! Depcrate_resolverimpl_452 {
() => {
// Module: crate::resolver
// Provides: {"impl_452"}
// Dependencies: {}
impl HasResolver for GenericDefId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { match self { GenericDefId :: FunctionId (inner) => inner . resolver (db) , GenericDefId :: AdtId (adt) => adt . resolver (db) , GenericDefId :: TraitId (inner) => inner . resolver (db) , GenericDefId :: TypeAliasId (inner) => inner . resolver (db) , GenericDefId :: ImplId (inner) => inner . resolver (db) , GenericDefId :: ConstId (inner) => inner . resolver (db) , GenericDefId :: StaticId (inner) => inner . resolver (db) , } } }
};
}
