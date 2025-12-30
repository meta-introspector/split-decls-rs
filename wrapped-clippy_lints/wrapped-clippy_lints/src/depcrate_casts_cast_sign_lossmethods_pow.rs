// Generated macro for METHODS_POW (const)
macro_rules! Depcrate_casts_cast_sign_lossMETHODS_POW {
() => {
// Module: crate::casts::cast_sign_loss
// Provides: {"METHODS_POW"}
// Dependencies: {}
# [doc = " A list of methods that act like `pow()`. See `pow_call_result_sign()` for details."] # [doc = ""] # [doc = " Methods that can overflow and return a negative value must not be included in this list,"] # [doc = " because casting their return values can still result in sign loss."] const METHODS_POW : & [Symbol] = & [sym :: pow , sym :: saturating_pow , sym :: checked_pow] ;
};
}
