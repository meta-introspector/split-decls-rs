macro_rules! deps {
    () => {
        HasModule!();
        DefDatabase!();
        ModuleId!();
    };
}

macro_rules! impl_733 {
    () => {
        deps!();
        impl HasModule for EnumVariantId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . lookup (db) . parent . module (db) } }
    };
}

impl_733!();