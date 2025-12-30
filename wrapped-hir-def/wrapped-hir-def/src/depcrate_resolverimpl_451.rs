// Generated macro for impl_451 (impl)
macro_rules! Depcrate_resolverimpl_451 {
() => {
// Module: crate::resolver
// Provides: {"impl_451"}
// Dependencies: {}
impl HasResolver for ItemContainerId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { match self { ItemContainerId :: ModuleId (it) => it . resolver (db) , ItemContainerId :: TraitId (it) => it . resolver (db) , ItemContainerId :: ImplId (it) => it . resolver (db) , ItemContainerId :: ExternBlockId (it) => it . resolver (db) , } } }
};
}
