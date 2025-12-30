// Generated macro for SnapshotContents (enum)
macro_rules! Depcrate_snapshotSnapshotContents {
() => {
// Module: crate::snapshot
// Provides: {"SnapshotContents"}
// Dependencies: {}
# [doc = " The contents of a Snapshot"] # [derive (Debug , Clone)] pub enum SnapshotContents { Text (TextSnapshotContents) , Binary (Rc < Vec < u8 > >) , }
};
}
