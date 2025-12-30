// Generated macro for source_root_durability (function)
macro_rules! Depcrate_changesource_root_durability {
() => {
// Module: crate::change
// Provides: {"source_root_durability"}
// Dependencies: {}
fn source_root_durability (source_root : & SourceRoot) -> Durability { if source_root . is_library { Durability :: MEDIUM } else { Durability :: LOW } }
};
}
