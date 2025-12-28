macro_rules! deps {
    () => {
        CrateRootModuleId!();
        ModuleId!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl PartialEq < CrateRootModuleId > for ModuleId { fn eq (& self , other : & CrateRootModuleId) -> bool { other == self } }
    };
}

impl_94!()