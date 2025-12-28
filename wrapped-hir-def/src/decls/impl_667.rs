macro_rules! deps {
    () => {
        CrateRootModuleId!();
        ModuleId!();
        DefMap!();
    };
}

macro_rules! impl_667 {
    () => {
        deps!();
        impl From < CrateRootModuleId > for ModuleId { fn from (CrateRootModuleId { krate } : CrateRootModuleId) -> Self { ModuleId { krate , block : None , local_id : DefMap :: ROOT } } }
    };
}

impl_667!()