// Generated macro for impl_363 (impl)
macro_rules! Depcrate_snapshotimpl_363 {
() => {
// Module: crate::snapshot
// Provides: {"impl_363"}
// Dependencies: {}
impl PartialEq for SnapshotContents { fn eq (& self , other : & Self) -> bool { match (self , other) { (SnapshotContents :: Text (this) , SnapshotContents :: Text (other)) => { if this . matches_latest (other) { true } else if this . matches_legacy (other) { elog ! ("{} {}\n{}" , style ("Snapshot test passes but the existing value is in a legacy format. Please run `cargo insta test --force-update-snapshots` to update to a newer format.") . yellow () . bold () , "Snapshot contents:" , this . to_string ()) ; true } else { false } } (SnapshotContents :: Binary (this) , SnapshotContents :: Binary (other)) => this == other , _ => false , } } }
};
}
