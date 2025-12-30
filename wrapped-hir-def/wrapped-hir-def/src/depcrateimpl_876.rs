// Generated macro for impl_876 (impl)
macro_rules! Depcrateimpl_876 {
() => {
// Module: crate
// Provides: {"impl_876"}
// Dependencies: {}
impl TryFrom < ModuleId > for CrateRootModuleId { type Error = () ; fn try_from (ModuleId { krate , block , local_id } : ModuleId) -> Result < Self , Self :: Error > { if block . is_none () && local_id == DefMap :: ROOT { Ok (CrateRootModuleId { krate }) } else { Err (()) } } }
};
}
