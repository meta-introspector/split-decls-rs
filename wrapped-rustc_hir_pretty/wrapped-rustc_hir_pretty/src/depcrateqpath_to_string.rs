// Generated macro for qpath_to_string (function)
macro_rules! Depcrateqpath_to_string {
() => {
// Module: crate
// Provides: {"qpath_to_string"}
// Dependencies: {}
pub fn qpath_to_string (ann : & dyn PpAnn , segment : & hir :: QPath < '_ >) -> String { to_string (ann , | s | s . print_qpath (segment , false)) }
};
}
