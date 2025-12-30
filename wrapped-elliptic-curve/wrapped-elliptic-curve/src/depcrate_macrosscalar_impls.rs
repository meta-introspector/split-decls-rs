// Generated macro for scalar_impls (macro)
macro_rules! Depcrate_macrosscalar_impls {
() => {
// Module: crate::macros
// Provides: {"scalar_impls"}
// Dependencies: {}
# [doc = " Writes all impls for scalar field types."] # [macro_export] macro_rules ! scalar_impls { ($ curve : path , $ scalar : ty) => { $ crate :: scalar_from_impls ! ($ curve , $ scalar) ; $ crate :: scalar_mul_impls ! ($ curve , $ scalar) ; } ; }
};
}
