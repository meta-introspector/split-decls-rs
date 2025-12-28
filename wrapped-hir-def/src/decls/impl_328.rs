macro_rules! deps {
    () => {
        CrateRootModuleId!();
        DefDatabase!();
        DefMap!();
        HasResolver!();
        Resolver!();
        ModuleItemMap!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl HasResolver for CrateRootModuleId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { let (def_map , local_def_map) = self . local_def_map (db) ; Resolver { scopes : vec ! [] , module_scope : ModuleItemMap { def_map , local_def_map , module_id : DefMap :: ROOT } , } } }
    };
}

impl_328!()