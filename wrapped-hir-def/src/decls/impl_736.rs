macro_rules! deps {
    () => {
        HasModule!();
        ModuleId!();
        DefDatabase!();
    };
}

macro_rules! impl_736 {
    () => {
        deps!();
        impl HasModule for ProcMacroId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . lookup (db) . container . into () } }
    };
}

impl_736!();