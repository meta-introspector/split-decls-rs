// Generated macro for MAX_DISTANCE (const)
macro_rules! Depcrate_matcher_support_edit_distanceMAX_DISTANCE {
() => {
// Module: crate::matcher_support::edit_distance
// Provides: {"MAX_DISTANCE"}
// Dependencies: {}
# [doc = " Maximum number of edits which can exist before [`edit_list`] falls back to a"] # [doc = " complete rewrite to produce the edit list."] # [doc = ""] # [doc = " Increasing this limit increases the accuracy of [`edit_list`] while"] # [doc = " quadratically increasing its worst-case runtime."] const MAX_DISTANCE : i32 = 50 ;
};
}
