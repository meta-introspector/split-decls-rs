// Generated macro for impl_300 (impl)
macro_rules! Depcrate_displayimpl_300 {
() => {
// Module: crate::display
// Provides: {"impl_300"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for Macro { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self . id { hir_def :: MacroId :: Macro2Id (_) => f . write_str ("macro") , hir_def :: MacroId :: MacroRulesId (_) => f . write_str ("macro_rules!") , hir_def :: MacroId :: ProcMacroId (_) => f . write_str ("proc_macro") , } ? ; write ! (f , " {}" , self . name (f . db) . display (f . db , f . edition ())) } }
};
}
