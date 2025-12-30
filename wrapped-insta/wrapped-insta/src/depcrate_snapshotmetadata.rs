// Generated macro for MetaData (struct)
macro_rules! Depcrate_snapshotMetaData {
() => {
// Module: crate::snapshot
// Provides: {"MetaData"}
// Dependencies: {}
# [doc = " Snapshot metadata information."] # [derive (Debug , Default , Clone , PartialEq)] pub struct MetaData { # [doc = " The source file (relative to workspace root)."] pub (crate) source : Option < String > , # [doc = " The source line, if available. This is used by pending snapshots, but trimmed"] # [doc = " before writing to the final `.snap` files in [`MetaData::trim_for_persistence`]."] pub (crate) assertion_line : Option < u32 > , # [doc = " Optional human readable (non formatted) snapshot description."] pub (crate) description : Option < String > , # [doc = " Optionally the expression that created the snapshot."] pub (crate) expression : Option < String > , # [doc = " An optional arbitrary structured info object."] pub (crate) info : Option < Content > , # [doc = " Reference to the input file."] pub (crate) input_file : Option < String > , # [doc = " The type of the snapshot (string or binary)."] pub (crate) snapshot_kind : SnapshotKind , }
};
}
