macro_rules! deps {
    () => {
        ModuleId!();
        HasModule!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl HasModule for ModuleId { # [inline] fn module (& self , _db : & dyn DefDatabase) -> ModuleId { * self } }
    };
}

impl_101!()