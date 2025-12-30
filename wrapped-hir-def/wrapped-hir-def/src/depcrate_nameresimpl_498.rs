// Generated macro for impl_498 (impl)
macro_rules! Depcrate_nameresimpl_498 {
() => {
// Module: crate::nameres
// Provides: {"impl_498"}
// Dependencies: {}
impl BlockRelativeModuleId { fn def_map (self , db : & dyn DefDatabase , krate : Crate) -> & DefMap { self . into_module (krate) . def_map (db) } fn into_module (self , krate : Crate) -> ModuleId { ModuleId { krate , block : self . block , local_id : self . local_id } } fn is_block_module (self) -> bool { self . block . is_some () && self . local_id == DefMap :: ROOT } }
};
}
