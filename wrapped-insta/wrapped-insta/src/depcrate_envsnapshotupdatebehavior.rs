// Generated macro for SnapshotUpdateBehavior (enum)
macro_rules! Depcrate_envSnapshotUpdateBehavior {
() => {
// Module: crate::env
// Provides: {"SnapshotUpdateBehavior"}
// Dependencies: {}
# [doc = " How snapshots are supposed to be updated"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum SnapshotUpdateBehavior { # [doc = " Snapshots are updated in-place"] InPlace , # [doc = " Snapshots are placed in a new file with a .new suffix"] NewFile , # [doc = " Snapshots are not updated at all."] NoUpdate , }
};
}
