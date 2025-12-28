macro_rules! deps {
    () => {
        HasModule!();
        ModuleId!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl HasModule for ProcMacroId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . lookup (db) . container . into () } }
    };
}

impl_164!()