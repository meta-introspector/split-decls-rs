// Generated macro for url_to_file_id (function)
macro_rules! Depcrate_global_stateurl_to_file_id {
() => {
// Module: crate::global_state
// Provides: {"url_to_file_id"}
// Dependencies: {}
# [doc = " Returns `None` if the file was excluded."] pub (crate) fn url_to_file_id (vfs : & vfs :: Vfs , url : & Url) -> anyhow :: Result < Option < FileId > > { let path = from_proto :: vfs_path (url) ? ; vfs_path_to_file_id (vfs , & path) }
};
}
