macro_rules! deps {
    () => {
        ItemContainerId!();
        HasResolver!();
        DefDatabase!();
        ModuleId!();
        Resolver!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl HasResolver for ItemContainerId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { match self { ItemContainerId :: ModuleId (it) => it . resolver (db) , ItemContainerId :: TraitId (it) => it . resolver (db) , ItemContainerId :: ImplId (it) => it . resolver (db) , ItemContainerId :: ExternBlockId (it) => it . resolver (db) , } } }
    };
}

impl_340!();