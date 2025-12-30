// Generated macro for impl_919 (impl)
macro_rules! Depcrateimpl_919 {
() => {
// Module: crate
// Provides: {"impl_919"}
// Dependencies: {}
impl From < ItemContainerId > for AttrDefId { fn from (acid : ItemContainerId) -> Self { match acid { ItemContainerId :: ModuleId (mid) => AttrDefId :: ModuleId (mid) , ItemContainerId :: ImplId (iid) => AttrDefId :: ImplId (iid) , ItemContainerId :: TraitId (tid) => AttrDefId :: TraitId (tid) , ItemContainerId :: ExternBlockId (id) => AttrDefId :: ExternBlockId (id) , } } }
};
}
