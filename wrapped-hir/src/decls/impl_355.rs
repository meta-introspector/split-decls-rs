macro_rules! deps {
    () => {
        Macro!();
        DeriveHelper!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl DeriveHelper { pub fn derive (& self) -> Macro { Macro { id : self . derive } } pub fn name (& self , db : & dyn HirDatabase) -> Name { match self . derive { makro @ MacroId :: Macro2Id (_) => db . attrs (makro . into ()) . parse_rustc_builtin_macro () . and_then (| (_ , helpers) | helpers . get (self . idx as usize) . cloned ()) , MacroId :: MacroRulesId (_) => None , makro @ MacroId :: ProcMacroId (_) => db . attrs (makro . into ()) . parse_proc_macro_derive () . and_then (| (_ , helpers) | helpers . get (self . idx as usize) . cloned ()) , } . unwrap_or_else (Name :: missing) } }
    };
}

impl_355!();