// Generated macro for cast_to_internal_error (function)
macro_rules! Depcrate_errorcast_to_internal_error {
() => {
// Module: crate::error
// Provides: {"cast_to_internal_error"}
// Dependencies: {}
# [doc = " Converts from external types to reqwest's"] # [doc = " internal equivalents."] # [doc = ""] # [doc = " Currently only is used for `tower::timeout::error::Elapsed`."] # [cfg (not (target_arch = "wasm32"))] pub (crate) fn cast_to_internal_error (error : BoxError) -> BoxError { if error . is :: < tower :: timeout :: error :: Elapsed > () { Box :: new (crate :: error :: TimedOut) as BoxError } else { error } }
};
}
