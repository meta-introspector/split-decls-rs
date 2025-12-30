// Generated macro for impl_517 (impl)
macro_rules! Depcrate_util_specialimpl_517 {
() => {
// Module: crate::util::special
// Provides: {"impl_517"}
// Dependencies: {}
impl Special { # [doc = " Create a new set of \"special\" state IDs with all IDs initialized to"] # [doc = " zero. The general idea here is that they will be updated and set to"] # [doc = " correct values later."] pub (crate) fn zero () -> Special { Special { max_special_id : StateID :: ZERO , max_match_id : StateID :: ZERO , start_unanchored_id : StateID :: ZERO , start_anchored_id : StateID :: ZERO , } } }
};
}
