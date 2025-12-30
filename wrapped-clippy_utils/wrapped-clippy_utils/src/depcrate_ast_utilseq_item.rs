// Generated macro for eq_item (function)
macro_rules! Depcrate_ast_utilseq_item {
() => {
// Module: crate::ast_utils
// Provides: {"eq_item"}
// Dependencies: {}
pub fn eq_item < K > (l : & Item < K > , r : & Item < K > , mut eq_kind : impl FnMut (& K , & K) -> bool) -> bool { over (& l . attrs , & r . attrs , eq_attr) && eq_vis (& l . vis , & r . vis) && eq_kind (& l . kind , & r . kind) }
};
}
