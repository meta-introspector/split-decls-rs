macro_rules! deps {
    () => {
        HasVisibility!();
        Macro!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl HasVisibility for Macro { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { match self . id { MacroId :: Macro2Id (id) => { let loc = id . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , id , source . map (| src | src . visibility ())) } MacroId :: MacroRulesId (_) => Visibility :: Public , MacroId :: ProcMacroId (_) => Visibility :: Public , } } }
    };
}

impl_318!();