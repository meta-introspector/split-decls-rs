// Generated macro for Difference (enum)
macro_rules! Depcrate_matcher_support_edit_distanceDifference {
() => {
// Module: crate::matcher_support::edit_distance
// Provides: {"Difference"}
// Dependencies: {}
# [doc = " The difference between two inputs as produced by [`edit_list`]."] # [derive (Debug)] pub (crate) enum Difference < T > { # [doc = " No differences were detected at all."] Equal , # [doc = " At most [`MAX_DISTANCE`] edits are required to convert one input to the"] # [doc = " other."] # [doc = ""] # [doc = " Contains the list of [`Edit`] to perform the transformation."] Editable (Vec < Edit < T > >) , # [doc = " More than [`MAX_DISTANCE`] edits are required to convert one input to"] # [doc = " the other."] # [doc = ""] # [doc = " The inputs are therefore considered unrelated and no edit list is"] # [doc = " provided."] Unrelated , }
};
}
