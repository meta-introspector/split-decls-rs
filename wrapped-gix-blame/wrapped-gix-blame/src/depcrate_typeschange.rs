// Generated macro for Change (enum)
macro_rules! Depcrate_typesChange {
() => {
// Module: crate::types
// Provides: {"Change"}
// Dependencies: {}
# [doc = " A single change between two blobs, or an unchanged region."] # [doc = ""] # [doc = " Line numbers refer to the file that is referred to as `after` or `NewOrDestination`, depending"] # [doc = " on the context."] # [derive (Clone , Debug , PartialEq)] pub enum Change { # [doc = " A range of tokens that wasn't changed."] Unchanged (Range < u32 >) , # [doc = " `(added_line_range, num_deleted_in_before)`"] AddedOrReplaced (Range < u32 > , u32) , # [doc = " `(line_to_start_deletion_at, num_deleted_in_before)`"] Deleted (u32 , u32) , }
};
}
