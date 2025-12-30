// Generated macro for FloatSIMDScalarUtils (trait)
macro_rules! Depcrate_distr_utilsFloatSIMDScalarUtils {
() => {
// Module: crate::distr::utils
// Provides: {"FloatSIMDScalarUtils"}
// Dependencies: {}
# [cfg (test)] pub (crate) trait FloatSIMDScalarUtils : FloatSIMDUtils { type Scalar ; fn replace (self , index : usize , new_value : Self :: Scalar) -> Self ; fn extract_lane (self , index : usize) -> Self :: Scalar ; }
};
}
