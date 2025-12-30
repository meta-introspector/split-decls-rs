// Generated macro for eq_variant (function)
macro_rules! Depcrate_ast_utilseq_variant {
() => {
// Module: crate::ast_utils
// Provides: {"eq_variant"}
// Dependencies: {}
pub fn eq_variant (l : & Variant , r : & Variant) -> bool { l . is_placeholder == r . is_placeholder && over (& l . attrs , & r . attrs , eq_attr) && eq_vis (& l . vis , & r . vis) && eq_id (l . ident , r . ident) && eq_variant_data (& l . data , & r . data) && both (l . disr_expr . as_ref () , r . disr_expr . as_ref () , | l , r | { eq_expr (& l . value , & r . value) }) }
};
}
