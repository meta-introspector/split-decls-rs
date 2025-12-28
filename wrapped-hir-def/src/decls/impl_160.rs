macro_rules! deps {
    () => {
        ModuleId!();
        HasModule!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl HasModule for TypeAliasId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { module_for_assoc_item_loc (db , * self) } }
    };
}

impl_160!()