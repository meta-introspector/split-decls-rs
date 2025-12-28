macro_rules! deps {
    () => {
        ModuleId!();
        CrateRootModuleId!();
    };
}

macro_rules! impl_666 {
    () => {
        deps!();
        impl PartialEq < CrateRootModuleId > for ModuleId { fn eq (& self , other : & CrateRootModuleId) -> bool { other == self } }
    };
}

impl_666!()