// Generated macro for DbgMacro (struct)
macro_rules! Depcrate_dbg_macroDbgMacro {
() => {
// Module: crate::dbg_macro
// Provides: {"DbgMacro"}
// Dependencies: {}
pub struct DbgMacro { allow_dbg_in_tests : bool , # [doc = " Tracks the `dbg!` macro callsites that are already checked."] checked_dbg_call_site : FxHashSet < Span > , # [doc = " Tracks the previous `SyntaxContext`, to avoid walking the same context chain."] prev_ctxt : SyntaxContext , }
};
}
