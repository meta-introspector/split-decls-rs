// Generated macro for iter_pin_mut (function)
macro_rules! Depcrate_future_join_alliter_pin_mut {
() => {
// Module: crate::future::join_all
// Provides: {"iter_pin_mut"}
// Dependencies: {}
pub (crate) fn iter_pin_mut < T > (slice : Pin < & mut [T] >) -> impl Iterator < Item = Pin < & mut T > > { unsafe { slice . get_unchecked_mut () } . iter_mut () . map (| t | unsafe { Pin :: new_unchecked (t) }) }
};
}
