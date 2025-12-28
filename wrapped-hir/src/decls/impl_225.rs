macro_rules! deps {
    () => {
        Macro!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for Macro { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self . id { hir_def :: MacroId :: Macro2Id (_) => f . write_str ("macro") , hir_def :: MacroId :: MacroRulesId (_) => f . write_str ("macro_rules!") , hir_def :: MacroId :: ProcMacroId (_) => f . write_str ("proc_macro") , } ? ; write ! (f , " {}" , self . name (f . db) . display (f . db , f . edition ())) } }
    };
}

impl_225!()