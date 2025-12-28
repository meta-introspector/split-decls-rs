macro_rules! deps {
    () => {
        MacroId!();
        ModuleId!();
        HasModule!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl HasModule for MacroId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match * self { MacroId :: MacroRulesId (it) => it . module (db) , MacroId :: Macro2Id (it) => it . module (db) , MacroId :: ProcMacroId (it) => it . module (db) , } } }
    };
}

impl_168!()