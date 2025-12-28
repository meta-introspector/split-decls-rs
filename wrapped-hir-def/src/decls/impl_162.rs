macro_rules! deps {
    () => {
        ModuleId!();
        HasModule!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl HasModule for MacroRulesId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . lookup (db) . container } }
    };
}

impl_162!()