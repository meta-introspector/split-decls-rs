macro_rules! deps {
    () => {
        ModuleId!();
        DefDatabase!();
        HasModule!();
    };
}

macro_rules! impl_730 {
    () => {
        deps!();
        impl HasModule for ConstId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { module_for_assoc_item_loc (db , * self) } }
    };
}

impl_730!()