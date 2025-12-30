// Generated macro for simd_element_to_bool (function)
macro_rules! Depcrate_helperssimd_element_to_bool {
() => {
// Module: crate::helpers
// Provides: {"simd_element_to_bool"}
// Dependencies: {}
pub (crate) fn simd_element_to_bool (elem : ImmTy < '_ >) -> InterpResult < '_ , bool > { assert ! (matches ! (elem . layout . ty . kind () , ty :: Int (_) | ty :: Uint (_)) , "SIMD mask element type must be an integer, but this is `{}`" , elem . layout . ty) ; let val = elem . to_scalar () . to_int (elem . layout . size) ? ; interp_ok (match val { 0 => false , - 1 => true , _ => throw_ub_format ! ("each element of a SIMD mask must be all-0-bits or all-1-bits") , }) }
};
}
