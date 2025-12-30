// Generated macro for impl_939 (impl)
macro_rules! Depcrateimpl_939 {
() => {
// Module: crate
// Provides: {"impl_939"}
// Dependencies: {}
impl HasModule for MacroId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match * self { MacroId :: MacroRulesId (it) => it . module (db) , MacroId :: Macro2Id (it) => it . module (db) , MacroId :: ProcMacroId (it) => it . module (db) , } } }
};
}
