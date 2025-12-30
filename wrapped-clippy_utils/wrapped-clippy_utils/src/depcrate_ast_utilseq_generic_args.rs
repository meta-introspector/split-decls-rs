// Generated macro for eq_generic_args (function)
macro_rules! Depcrate_ast_utilseq_generic_args {
() => {
// Module: crate::ast_utils
// Provides: {"eq_generic_args"}
// Dependencies: {}
pub fn eq_generic_args (l : & GenericArgs , r : & GenericArgs) -> bool { match (l , r) { (AngleBracketed (l) , AngleBracketed (r)) => over (& l . args , & r . args , eq_angle_arg) , (Parenthesized (l) , Parenthesized (r)) => { over (& l . inputs , & r . inputs , | l , r | eq_ty (l , r)) && eq_fn_ret_ty (& l . output , & r . output) } , _ => false , } }
};
}
