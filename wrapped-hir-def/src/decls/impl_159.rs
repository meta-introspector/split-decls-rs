macro_rules! deps {
    () => {
        HasModule!();
        ModuleId!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl HasModule for StaticId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { module_for_assoc_item_loc (db , * self) } }
    };
}

impl_159!()