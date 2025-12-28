macro_rules! deps {
    () => {
        ModuleId!();
        HasModule!();
        DefDatabase!();
    };
}

macro_rules! impl_729 {
    () => {
        deps!();
        impl HasModule for FunctionId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { module_for_assoc_item_loc (db , * self) } }
    };
}

impl_729!()