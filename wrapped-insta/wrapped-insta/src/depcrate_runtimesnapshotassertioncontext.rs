// Generated macro for SnapshotAssertionContext (struct)
macro_rules! Depcrate_runtimeSnapshotAssertionContext {
() => {
// Module: crate::runtime
// Provides: {"SnapshotAssertionContext"}
// Dependencies: {}
# [doc = " The context around a snapshot, such as the reference value, location, etc."] # [doc = " (but not including the generated value). Responsible for saving the"] # [doc = " snapshot."] # [derive (Debug)] struct SnapshotAssertionContext < 'a > { tool_config : Arc < ToolConfig > , workspace : & 'a Path , module_path : & 'a str , snapshot_name : Option < Cow < 'a , str > > , snapshot_file : Option < PathBuf > , duplication_key : Option < String > , old_snapshot : Option < Snapshot > , pending_snapshots_path : Option < PathBuf > , assertion_file : & 'a str , assertion_line : u32 , is_doctest : bool , snapshot_kind : SnapshotKind , }
};
}
