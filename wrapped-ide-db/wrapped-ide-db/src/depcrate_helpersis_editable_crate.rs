// Generated macro for is_editable_crate (function)
macro_rules! Depcrate_helpersis_editable_crate {
() => {
// Module: crate::helpers
// Provides: {"is_editable_crate"}
// Dependencies: {}
pub fn is_editable_crate (krate : Crate , db : & RootDatabase) -> bool { let root_file = krate . root_file (db) ; let source_root_id = db . file_source_root (root_file) . source_root_id (db) ; ! db . source_root (source_root_id) . source_root (db) . is_library }
};
}
