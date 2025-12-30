// Generated macro for file_id (function)
macro_rules! Depcrate_integrated_benchmarksfile_id {
() => {
// Module: crate::integrated_benchmarks
// Provides: {"file_id"}
// Dependencies: {}
# [track_caller] fn file_id (vfs : & vfs :: Vfs , path : & VfsPath) -> vfs :: FileId { match vfs . file_id (path) { Some ((file_id , vfs :: FileExcluded :: No)) => file_id , None | Some ((_ , vfs :: FileExcluded :: Yes)) => panic ! ("can't find virtual file for {path}") , } }
};
}
