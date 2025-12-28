macro_rules! deps {
    () => {
        Macro!();
        HasSource!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl HasSource for Macro { type Ast = Either < ast :: Macro , ast :: Fn > ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { match self . id { MacroId :: Macro2Id (it) => { Some (it . lookup (db) . source (db) . map (ast :: Macro :: MacroDef) . map (Either :: Left)) } MacroId :: MacroRulesId (it) => { Some (it . lookup (db) . source (db) . map (ast :: Macro :: MacroRules) . map (Either :: Left)) } MacroId :: ProcMacroId (it) => Some (it . lookup (db) . source (db) . map (Either :: Right)) , } } }
    };
}

impl_60!();