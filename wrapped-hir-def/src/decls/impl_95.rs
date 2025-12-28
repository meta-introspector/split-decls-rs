macro_rules! deps {
    () => {
        ModuleId!();
        CrateRootModuleId!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl From < CrateRootModuleId > for ModuleId { fn from (CrateRootModuleId { krate } : CrateRootModuleId) -> Self { ModuleId { krate , block : None , local_id : DefMap :: ROOT } } }
    };
}

impl_95!()