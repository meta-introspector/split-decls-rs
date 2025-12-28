macro_rules! deps {
    () => {
        HasModule!();
        ModuleId!();
        DefDatabase!();
    };
}

macro_rules! impl_673 {
    () => {
        deps!();
        impl HasModule for ModuleId { # [inline] fn module (& self , _db : & dyn DefDatabase) -> ModuleId { * self } }
    };
}

impl_673!();