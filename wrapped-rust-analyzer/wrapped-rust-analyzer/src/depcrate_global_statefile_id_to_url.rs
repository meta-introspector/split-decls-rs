// Generated macro for file_id_to_url (function)
macro_rules! Depcrate_global_statefile_id_to_url {
() => {
// Module: crate::global_state
// Provides: {"file_id_to_url"}
// Dependencies: {}
pub (crate) fn file_id_to_url (vfs : & vfs :: Vfs , id : FileId) -> Url { let path = vfs . file_path (id) ; let path = path . as_path () . unwrap () ; url_from_abs_path (path) }
};
}
