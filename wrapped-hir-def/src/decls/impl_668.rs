macro_rules! deps {
    () => {
        ModuleId!();
        CrateRootModuleId!();
        ModuleDefId!();
    };
}

macro_rules! impl_668 {
    () => {
        deps!();
        impl From < CrateRootModuleId > for ModuleDefId { fn from (value : CrateRootModuleId) -> Self { ModuleDefId :: ModuleId (value . into ()) } }
    };
}

impl_668!()