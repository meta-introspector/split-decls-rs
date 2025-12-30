// Generated macro for impl_936 (impl)
macro_rules! Depcrateimpl_936 {
() => {
// Module: crate
// Provides: {"impl_936"}
// Dependencies: {}
impl HasModule for ItemContainerId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match * self { ItemContainerId :: ModuleId (it) => it , ItemContainerId :: ImplId (it) => it . module (db) , ItemContainerId :: TraitId (it) => it . module (db) , ItemContainerId :: ExternBlockId (it) => it . module (db) , } } }
};
}
