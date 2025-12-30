// Generated macro for eq_path_seg (function)
macro_rules! Depcrate_ast_utilseq_path_seg {
() => {
// Module: crate::ast_utils
// Provides: {"eq_path_seg"}
// Dependencies: {}
pub fn eq_path_seg (l : & PathSegment , r : & PathSegment) -> bool { eq_id (l . ident , r . ident) && both (l . args . as_ref () , r . args . as_ref () , | l , r | eq_generic_args (l , r)) }
};
}
