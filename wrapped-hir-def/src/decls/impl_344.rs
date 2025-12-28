macro_rules! deps {
    () => {
        MacroId!();
        HasResolver!();
        Resolver!();
        DefDatabase!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl HasResolver for MacroId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { match self { MacroId :: Macro2Id (it) => it . resolver (db) , MacroId :: MacroRulesId (it) => it . resolver (db) , MacroId :: ProcMacroId (it) => it . resolver (db) , } } }
    };
}

impl_344!()