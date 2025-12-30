// Generated macro for eq_attr_args (function)
macro_rules! Depcrate_ast_utilseq_attr_args {
() => {
// Module: crate::ast_utils
// Provides: {"eq_attr_args"}
// Dependencies: {}
pub fn eq_attr_args (l : & AttrArgs , r : & AttrArgs) -> bool { use AttrArgs :: * ; match (l , r) { (Empty , Empty) => true , (Delimited (la) , Delimited (ra)) => eq_delim_args (la , ra) , (Eq { eq_span : _ , expr : le } , Eq { eq_span : _ , expr : re }) => eq_expr (le , re) , _ => false , } }
};
}
