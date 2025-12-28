macro_rules! deps {
    () => {
        DefDatabase!();
        ModuleId!();
        HasModule!();
    };
}

macro_rules! impl_673 {
    () => {
        deps!();
        impl HasModule for ModuleId { # [inline] fn module (& self , _db : & dyn DefDatabase) -> ModuleId { * self } }
    };
}

impl_673!()