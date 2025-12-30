// Generated macro for snapshot_update_behavior (function)
macro_rules! Depcrate_envsnapshot_update_behavior {
() => {
// Module: crate::env
// Provides: {"snapshot_update_behavior"}
// Dependencies: {}
# [doc = " Returns the intended snapshot update behavior."] pub fn snapshot_update_behavior (tool_config : & ToolConfig , unseen : bool) -> SnapshotUpdateBehavior { match tool_config . snapshot_update () { SnapshotUpdate :: Always => SnapshotUpdateBehavior :: InPlace , SnapshotUpdate :: Auto => { if is_ci () { SnapshotUpdateBehavior :: NoUpdate } else { SnapshotUpdateBehavior :: NewFile } } SnapshotUpdate :: Unseen => { if unseen { SnapshotUpdateBehavior :: NewFile } else { SnapshotUpdateBehavior :: InPlace } } SnapshotUpdate :: New => SnapshotUpdateBehavior :: NewFile , SnapshotUpdate :: No => SnapshotUpdateBehavior :: NoUpdate , SnapshotUpdate :: Force => SnapshotUpdateBehavior :: InPlace , } }
};
}
