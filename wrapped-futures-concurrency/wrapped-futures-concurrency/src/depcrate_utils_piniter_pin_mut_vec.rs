// Generated macro for iter_pin_mut_vec (function)
macro_rules! Depcrate_utils_piniter_pin_mut_vec {
() => {
// Module: crate::utils::pin
// Provides: {"iter_pin_mut_vec"}
// Dependencies: {}
# [cfg (feature = "alloc")] pub (crate) fn iter_pin_mut_vec < T > (slice : Pin < & mut Vec < T > >) -> impl Iterator < Item = Pin < & mut T > > { unsafe { slice . get_unchecked_mut () } . iter_mut () . map (| t | unsafe { Pin :: new_unchecked (t) }) }
};
}
