// Generated macro for first_dbg_macro_in_expansion (function)
macro_rules! Depcrate_dbg_macrofirst_dbg_macro_in_expansion {
() => {
// Module: crate::dbg_macro
// Provides: {"first_dbg_macro_in_expansion"}
// Dependencies: {}
fn first_dbg_macro_in_expansion (cx : & LateContext < '_ > , span : Span) -> Option < MacroCall > { macro_backtrace (span) . find (| mc | cx . tcx . is_diagnostic_item (sym :: dbg_macro , mc . def_id)) }
};
}
