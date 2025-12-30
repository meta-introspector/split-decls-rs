// Generated macro for eq_attr (function)
macro_rules! Depcrate_ast_utilseq_attr {
() => {
// Module: crate::ast_utils
// Provides: {"eq_attr"}
// Dependencies: {}
pub fn eq_attr (l : & Attribute , r : & Attribute) -> bool { use AttrKind :: * ; l . style == r . style && match (& l . kind , & r . kind) { (DocComment (l1 , l2) , DocComment (r1 , r2)) => l1 == r1 && l2 == r2 , (Normal (l) , Normal (r)) => eq_path (& l . item . path , & r . item . path) && eq_attr_args (& l . item . args , & r . item . args) , _ => false , } }
};
}
