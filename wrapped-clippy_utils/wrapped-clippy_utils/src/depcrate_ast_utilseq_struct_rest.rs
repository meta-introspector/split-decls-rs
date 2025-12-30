// Generated macro for eq_struct_rest (function)
macro_rules! Depcrate_ast_utilseq_struct_rest {
() => {
// Module: crate::ast_utils
// Provides: {"eq_struct_rest"}
// Dependencies: {}
pub fn eq_struct_rest (l : & StructRest , r : & StructRest) -> bool { match (l , r) { (StructRest :: Base (lb) , StructRest :: Base (rb)) => eq_expr (lb , rb) , (StructRest :: Rest (_) , StructRest :: Rest (_)) | (StructRest :: None , StructRest :: None) => true , _ => false , } }
};
}
