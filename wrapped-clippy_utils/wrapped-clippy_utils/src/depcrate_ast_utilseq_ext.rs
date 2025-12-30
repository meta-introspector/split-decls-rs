// Generated macro for eq_ext (function)
macro_rules! Depcrate_ast_utilseq_ext {
() => {
// Module: crate::ast_utils
// Provides: {"eq_ext"}
// Dependencies: {}
pub fn eq_ext (l : & Extern , r : & Extern) -> bool { use Extern :: * ; match (l , r) { (None , None) | (Implicit (_) , Implicit (_)) => true , (Explicit (l , _) , Explicit (r , _)) => eq_str_lit (l , r) , _ => false , } }
};
}
