// Generated macro for impl_461 (impl)
macro_rules! Depcrate_resolverimpl_461 {
() => {
// Module: crate::resolver
// Provides: {"impl_461"}
// Dependencies: {}
impl HasResolver for MacroId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { match self { MacroId :: Macro2Id (it) => it . resolver (db) , MacroId :: MacroRulesId (it) => it . resolver (db) , MacroId :: ProcMacroId (it) => it . resolver (db) , } } }
};
}
