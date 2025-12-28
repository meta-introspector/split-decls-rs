macro_rules! deps {
    () => {
        ModuleId!();
        HasModule!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl HasModule for FunctionId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { module_for_assoc_item_loc (db , * self) } }
    };
}

impl_157!()