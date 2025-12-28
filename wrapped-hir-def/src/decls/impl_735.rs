macro_rules! deps {
    () => {
        HasModule!();
        DefDatabase!();
        ModuleId!();
    };
}

macro_rules! impl_735 {
    () => {
        deps!();
        impl HasModule for Macro2Id { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . lookup (db) . container } }
    };
}

impl_735!()