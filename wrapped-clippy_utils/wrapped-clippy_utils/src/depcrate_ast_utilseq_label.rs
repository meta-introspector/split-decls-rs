// Generated macro for eq_label (function)
macro_rules! Depcrate_ast_utilseq_label {
() => {
// Module: crate::ast_utils
// Provides: {"eq_label"}
// Dependencies: {}
pub fn eq_label (l : Option < & Label > , r : Option < & Label >) -> bool { both (l , r , | l , r | eq_id (l . ident , r . ident)) }
};
}
