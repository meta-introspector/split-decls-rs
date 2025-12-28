macro_rules! deps {
    () => {
        AttrDefId!();
        ItemContainerId!();
        ModuleId!();
    };
}

macro_rules! impl_720 {
    () => {
        deps!();
        impl From < ItemContainerId > for AttrDefId { fn from (acid : ItemContainerId) -> Self { match acid { ItemContainerId :: ModuleId (mid) => AttrDefId :: ModuleId (mid) , ItemContainerId :: ImplId (iid) => AttrDefId :: ImplId (iid) , ItemContainerId :: TraitId (tid) => AttrDefId :: TraitId (tid) , ItemContainerId :: ExternBlockId (id) => AttrDefId :: ExternBlockId (id) , } } }
    };
}

impl_720!()