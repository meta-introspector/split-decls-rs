// Generated macro for eq_vis (function)
macro_rules! Depcrate_ast_utilseq_vis {
() => {
// Module: crate::ast_utils
// Provides: {"eq_vis"}
// Dependencies: {}
pub fn eq_vis (l : & Visibility , r : & Visibility) -> bool { use VisibilityKind :: * ; match (& l . kind , & r . kind) { (Public , Public) | (Inherited , Inherited) => true , (Restricted { path : l , .. } , Restricted { path : r , .. }) => eq_path (l , r) , _ => false , } }
};
}
