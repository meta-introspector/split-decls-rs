// Generated macro for source_root_crates (function)
macro_rules! Depcratesource_root_crates {
() => {
// Module: crate
// Provides: {"source_root_crates"}
// Dependencies: {}
fn source_root_crates (db : & dyn RootQueryDb , id : SourceRootId) -> Arc < [Crate] > { let crates = db . all_crates () ; crates . iter () . copied () . filter (| & krate | { let root_file = krate . data (db) . root_file_id ; db . file_source_root (root_file) . source_root_id (db) == id }) . collect () }
};
}
