// Generated macro for SnapshotContainer (struct)
macro_rules! Depcrate_containerSnapshotContainer {
() => {
// Module: crate::container
// Provides: {"SnapshotContainer"}
// Dependencies: {}
# [doc = " A snapshot and its immediate context, which loads & saves the snapshot. It"] # [doc = " holds either a single file snapshot, or all the inline snapshots from a"] # [doc = " single rust file."] # [derive (Debug , Clone)] pub (crate) struct SnapshotContainer { pending_path : PathBuf , target_path : PathBuf , kind : TextSnapshotKind , snapshots : Vec < PendingSnapshot > , patcher : Option < FilePatcher > , }
};
}
