macro_rules! deps {
    () => {
        CrateRootModuleId!();
        ModuleDefId!();
        ModuleId!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl From < CrateRootModuleId > for ModuleDefId { fn from (value : CrateRootModuleId) -> Self { ModuleDefId :: ModuleId (value . into ()) } }
    };
}

impl_96!()