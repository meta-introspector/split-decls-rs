// Generated macro for ty_to_string (function)
macro_rules! Depcratety_to_string {
() => {
// Module: crate
// Provides: {"ty_to_string"}
// Dependencies: {}
pub fn ty_to_string (ann : & dyn PpAnn , ty : & hir :: Ty < '_ >) -> String { to_string (ann , | s | s . print_type (ty)) }
};
}
