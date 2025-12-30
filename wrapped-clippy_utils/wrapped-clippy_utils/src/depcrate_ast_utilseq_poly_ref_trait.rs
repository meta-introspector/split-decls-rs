// Generated macro for eq_poly_ref_trait (function)
macro_rules! Depcrate_ast_utilseq_poly_ref_trait {
() => {
// Module: crate::ast_utils
// Provides: {"eq_poly_ref_trait"}
// Dependencies: {}
pub fn eq_poly_ref_trait (l : & PolyTraitRef , r : & PolyTraitRef) -> bool { l . modifiers == r . modifiers && eq_path (& l . trait_ref . path , & r . trait_ref . path) && over (& l . bound_generic_params , & r . bound_generic_params , | l , r | { eq_generic_param (l , r) }) }
};
}
