// Generated macro for pat_to_string (function)
macro_rules! Depcratepat_to_string {
() => {
// Module: crate
// Provides: {"pat_to_string"}
// Dependencies: {}
pub fn pat_to_string (ann : & dyn PpAnn , pat : & hir :: Pat < '_ >) -> String { to_string (ann , | s | s . print_pat (pat)) }
};
}
