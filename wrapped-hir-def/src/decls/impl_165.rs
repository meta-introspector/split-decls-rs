macro_rules! deps {
    () => {
        ItemContainerId!();
        ModuleId!();
        HasModule!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl HasModule for ItemContainerId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match * self { ItemContainerId :: ModuleId (it) => it , ItemContainerId :: ImplId (it) => it . module (db) , ItemContainerId :: TraitId (it) => it . module (db) , ItemContainerId :: ExternBlockId (it) => it . module (db) , } } }
    };
}

impl_165!()