// Generated macro for SemanticsImpl (struct)
macro_rules! Depcrate_semanticsSemanticsImpl {
() => {
// Module: crate::semantics
// Provides: {"SemanticsImpl"}
// Dependencies: {}
pub struct SemanticsImpl < 'db > { pub db : & 'db dyn HirDatabase , s2d_cache : RefCell < SourceToDefCache > , # [doc = " MacroCall to its expansion's MacroCallId cache"] macro_call_cache : RefCell < FxHashMap < InFile < ast :: MacroCall > , MacroCallId > > , }
};
}
