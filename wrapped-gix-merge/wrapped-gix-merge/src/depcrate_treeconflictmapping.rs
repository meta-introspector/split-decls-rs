// Generated macro for ConflictMapping (enum)
macro_rules! Depcrate_treeConflictMapping {
() => {
// Module: crate::tree
// Provides: {"ConflictMapping"}
// Dependencies: {}
# [doc = " A utility to help define which side is what in the [`Conflict`] type."] # [derive (Debug , Clone , Copy , Eq , PartialEq)] enum ConflictMapping { # [doc = " The sides are as described in the field documentation, i.e. `ours` is `ours`."] Original , # [doc = " The sides are the opposite of the field documentation. i.e. `ours` is `theirs` and `theirs` is `ours`."] Swapped , }
};
}
