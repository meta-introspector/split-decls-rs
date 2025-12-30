// Generated macro for PendingInlineSnapshot (struct)
macro_rules! Depcrate_snapshotPendingInlineSnapshot {
() => {
// Module: crate::snapshot
// Provides: {"PendingInlineSnapshot"}
// Dependencies: {}
# [doc = " Holds a pending inline snapshot loaded from a json file or read from an assert"] # [doc = " macro (doesn't write to the rust file, which is done by `cargo-insta`)"] # [derive (Debug)] pub struct PendingInlineSnapshot { pub run_id : String , pub line : u32 , pub new : Option < Snapshot > , pub old : Option < Snapshot > , }
};
}
