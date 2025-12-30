// Generated macro for SnapshotPrinter (struct)
macro_rules! Depcrate_outputSnapshotPrinter {
() => {
// Module: crate::output
// Provides: {"SnapshotPrinter"}
// Dependencies: {}
# [doc = " Snapshot printer utility."] pub struct SnapshotPrinter < 'a > { workspace_root : & 'a Path , old_snapshot : Option < & 'a Snapshot > , new_snapshot : & 'a Snapshot , old_snapshot_hint : & 'a str , new_snapshot_hint : & 'a str , show_info : bool , show_diff : bool , title : Option < & 'a str > , line : Option < u32 > , snapshot_file : Option < & 'a Path > , }
};
}
