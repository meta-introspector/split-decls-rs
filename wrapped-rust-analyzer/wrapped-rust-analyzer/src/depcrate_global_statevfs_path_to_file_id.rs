// Generated macro for vfs_path_to_file_id (function)
macro_rules! Depcrate_global_statevfs_path_to_file_id {
() => {
// Module: crate::global_state
// Provides: {"vfs_path_to_file_id"}
// Dependencies: {}
# [doc = " Returns `None` if the file was excluded."] pub (crate) fn vfs_path_to_file_id (vfs : & vfs :: Vfs , vfs_path : & VfsPath ,) -> anyhow :: Result < Option < FileId > > { let (file_id , excluded) = vfs . file_id (vfs_path) . ok_or_else (| | anyhow :: format_err ! ("file not found: {vfs_path}")) ? ; match excluded { vfs :: FileExcluded :: Yes => Ok (None) , vfs :: FileExcluded :: No => Ok (Some (file_id)) , } }
};
}
