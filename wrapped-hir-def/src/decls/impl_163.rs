macro_rules! deps {
    () => {
        ModuleId!();
        HasModule!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl HasModule for Macro2Id { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . lookup (db) . container } }
    };
}

impl_163!()