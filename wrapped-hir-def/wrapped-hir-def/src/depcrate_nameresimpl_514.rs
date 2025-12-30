// Generated macro for impl_514 (impl)
macro_rules! Depcrate_nameresimpl_514 {
() => {
// Module: crate::nameres
// Provides: {"impl_514"}
// Dependencies: {}
impl MacroSubNs { fn from_id (db : & dyn DefDatabase , macro_id : MacroId) -> Self { let expander = match macro_id { MacroId :: Macro2Id (it) => it . lookup (db) . expander , MacroId :: MacroRulesId (it) => it . lookup (db) . expander , MacroId :: ProcMacroId (it) => { return match it . lookup (db) . kind { ProcMacroKind :: CustomDerive | ProcMacroKind :: Attr => Self :: Attr , ProcMacroKind :: Bang => Self :: Bang , } ; } } ; match expander { MacroExpander :: Declarative | MacroExpander :: BuiltIn (_) | MacroExpander :: BuiltInEager (_) => Self :: Bang , MacroExpander :: BuiltInAttr (_) | MacroExpander :: BuiltInDerive (_) => Self :: Attr , } } }
};
}
