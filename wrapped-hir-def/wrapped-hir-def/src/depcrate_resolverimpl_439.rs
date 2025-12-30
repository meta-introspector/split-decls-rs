// Generated macro for impl_439 (impl)
macro_rules! Depcrate_resolverimpl_439 {
() => {
// Module: crate::resolver
// Provides: {"impl_439"}
// Dependencies: {}
impl HasResolver for CrateRootModuleId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { let (def_map , local_def_map) = self . local_def_map (db) ; Resolver { scopes : vec ! [] , module_scope : ModuleItemMap { def_map , local_def_map , module_id : DefMap :: ROOT } , } } }
};
}
