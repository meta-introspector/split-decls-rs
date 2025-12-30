// Generated macro for eq_maybe_qself (function)
macro_rules! Depcrate_ast_utilseq_maybe_qself {
() => {
// Module: crate::ast_utils
// Provides: {"eq_maybe_qself"}
// Dependencies: {}
pub fn eq_maybe_qself (l : Option < & QSelf > , r : Option < & QSelf >) -> bool { match (l , r) { (Some (l) , Some (r)) => eq_qself (l , r) , (None , None) => true , _ => false , } }
};
}
