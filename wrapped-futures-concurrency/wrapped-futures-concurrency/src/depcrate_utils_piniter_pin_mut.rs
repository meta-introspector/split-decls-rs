// Generated macro for iter_pin_mut (function)
macro_rules! Depcrate_utils_piniter_pin_mut {
() => {
// Module: crate::utils::pin
// Provides: {"iter_pin_mut"}
// Dependencies: {}
pub (crate) fn iter_pin_mut < T > (slice : Pin < & mut [T] >) -> impl Iterator < Item = Pin < & mut T > > { unsafe { slice . get_unchecked_mut () } . iter_mut () . map (| t | unsafe { Pin :: new_unchecked (t) }) }
};
}
