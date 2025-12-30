// Generated macro for impl_41 (impl)
macro_rules! Depcrate_attrs_data_structuresimpl_41 {
() => {
// Module: crate::attrs::data_structures
// Provides: {"impl_41"}
// Dependencies: {}
impl < ModId > StrippedCfgItem < ModId > { pub fn map_mod_id < New > (self , f : impl FnOnce (ModId) -> New) -> StrippedCfgItem < New > { StrippedCfgItem { parent_module : f (self . parent_module) , ident : self . ident , cfg : self . cfg } } }
};
}
