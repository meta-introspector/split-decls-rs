// Generated macro for file_text_durability (function)
macro_rules! Depcrate_changefile_text_durability {
() => {
// Module: crate::change
// Provides: {"file_text_durability"}
// Dependencies: {}
fn file_text_durability (source_root : & SourceRoot) -> Durability { if source_root . is_library { Durability :: HIGH } else { Durability :: LOW } }
};
}
