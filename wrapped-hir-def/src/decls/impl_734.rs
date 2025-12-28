macro_rules! deps {
    () => {
        HasModule!();
        DefDatabase!();
        ModuleId!();
    };
}

macro_rules! impl_734 {
    () => {
        deps!();
        impl HasModule for MacroRulesId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . lookup (db) . container } }
    };
}

impl_734!()