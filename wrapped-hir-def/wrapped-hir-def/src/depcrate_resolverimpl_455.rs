// Generated macro for impl_455 (impl)
macro_rules! Depcrate_resolverimpl_455 {
() => {
// Module: crate::resolver
// Provides: {"impl_455"}
// Dependencies: {}
impl HasResolver for MacroId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { match self { MacroId :: Macro2Id (it) => it . resolver (db) , MacroId :: MacroRulesId (it) => it . resolver (db) , MacroId :: ProcMacroId (it) => it . resolver (db) , } } }
};
}
