macro_rules! deps {
    () => {
        HasModule!();
        ModuleId!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl HasModule for EnumVariantId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . lookup (db) . parent . module (db) } }
    };
}

impl_161!()