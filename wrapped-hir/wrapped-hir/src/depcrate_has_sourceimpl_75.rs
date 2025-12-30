// Generated macro for impl_75 (impl)
macro_rules! Depcrate_has_sourceimpl_75 {
() => {
// Module: crate::has_source
// Provides: {"impl_75"}
// Dependencies: {}
impl HasSource for Macro { type Ast = Either < ast :: Macro , ast :: Fn > ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { match self . id { MacroId :: Macro2Id (it) => { Some (it . lookup (db) . source (db) . map (ast :: Macro :: MacroDef) . map (Either :: Left)) } MacroId :: MacroRulesId (it) => { Some (it . lookup (db) . source (db) . map (ast :: Macro :: MacroRules) . map (Either :: Left)) } MacroId :: ProcMacroId (it) => Some (it . lookup (db) . source (db) . map (Either :: Right)) , } } }
};
}
