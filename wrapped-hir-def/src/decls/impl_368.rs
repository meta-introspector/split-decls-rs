macro_rules! deps {
    () => {
        ModuleData!();
        DefMap!();
        LocalModuleId!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        impl std :: ops :: Index < LocalModuleId > for DefMap { type Output = ModuleData ; fn index (& self , id : LocalModuleId) -> & ModuleData { & self . modules [id] } }
    };
}

impl_368!();