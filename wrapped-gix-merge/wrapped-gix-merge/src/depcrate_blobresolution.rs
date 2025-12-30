// Generated macro for Resolution (enum)
macro_rules! Depcrate_blobResolution {
() => {
// Module: crate::blob
// Provides: {"Resolution"}
// Dependencies: {}
# [doc = " Define if a merge is conflicted or not."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum Resolution { # [doc = " Everything could be resolved during the merge, and there was no conflict."] Complete , # [doc = " Conflicts were resolved automatically, even thought the result is complete"] # [doc = " and free of conflict markers."] # [doc = " This can only be the case for text-file content merges."] CompleteWithAutoResolvedConflict , # [doc = " A conflict is still present in the form of conflict markers."] Conflict , }
};
}
