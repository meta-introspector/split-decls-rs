// Generated macro for is_iterator_singleton (function)
macro_rules! Depcrate_fn_ctxt_adjust_fulfillment_errorsis_iterator_singleton {
() => {
// Module: crate::fn_ctxt::adjust_fulfillment_errors
// Provides: {"is_iterator_singleton"}
// Dependencies: {}
# [doc = " Returns `Some(iterator.next())` if it has exactly one item, and `None` otherwise."] fn is_iterator_singleton < T > (mut iterator : impl Iterator < Item = T >) -> Option < T > { match (iterator . next () , iterator . next ()) { (_ , Some (_)) => None , (first , _) => first , } }
};
}
