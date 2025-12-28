macro_rules! deps {
    () => {
        HasModule!();
        DefDatabase!();
        ModuleId!();
    };
}

macro_rules! impl_732 {
    () => {
        deps!();
        impl HasModule for TypeAliasId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { module_for_assoc_item_loc (db , * self) } }
    };
}

impl_732!()