// Generated macro for impl_864 (impl)
macro_rules! Depcrateimpl_864 {
() => {
// Module: crate
// Provides: {"impl_864"}
// Dependencies: {}
impl PartialEq < ModuleId > for CrateRootModuleId { fn eq (& self , other : & ModuleId) -> bool { other . block . is_none () && other . local_id == DefMap :: ROOT && self . krate == other . krate } }
};
}
