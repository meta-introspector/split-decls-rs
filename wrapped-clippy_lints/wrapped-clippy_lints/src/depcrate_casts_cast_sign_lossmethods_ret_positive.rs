// Generated macro for METHODS_RET_POSITIVE (const)
macro_rules! Depcrate_casts_cast_sign_lossMETHODS_RET_POSITIVE {
() => {
// Module: crate::casts::cast_sign_loss
// Provides: {"METHODS_RET_POSITIVE"}
// Dependencies: {}
# [doc = " A list of methods that can never return a negative value."] # [doc = " Includes methods that panic rather than returning a negative value."] # [doc = ""] # [doc = " Methods that can overflow and return a negative value must not be included in this list,"] # [doc = " because casting their return values can still result in sign loss."] const METHODS_RET_POSITIVE : & [Symbol] = & [sym :: checked_abs , sym :: saturating_abs , sym :: isqrt , sym :: checked_isqrt , sym :: rem_euclid , sym :: checked_rem_euclid , sym :: wrapping_rem_euclid ,] ;
};
}
