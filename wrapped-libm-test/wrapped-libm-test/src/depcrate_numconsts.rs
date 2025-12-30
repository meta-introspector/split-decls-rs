// Generated macro for Consts (struct)
macro_rules! Depcrate_numConsts {
() => {
// Module: crate::num
// Provides: {"Consts"}
// Dependencies: {}
# [doc = " Extra constants that are useful for tests."] # [derive (Debug , Clone , Copy)] pub struct Consts < F > { # [doc = " The default quiet NaN, which is also the minimum quiet NaN."] pub pos_nan : F , # [doc = " The default quiet NaN with negative sign."] pub neg_nan : F , # [doc = " NaN with maximum (unsigned) significand to be a quiet NaN. The significand is saturated."] pub max_qnan : F , # [doc = " NaN with minimum (unsigned) significand to be a signaling NaN."] pub min_snan : F , # [doc = " NaN with maximum (unsigned) significand to be a signaling NaN."] pub max_snan : F , pub neg_max_qnan : F , pub neg_min_snan : F , pub neg_max_snan : F , }
};
}
