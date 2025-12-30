// Generated macro for read_file_by_commit (function)
macro_rules! Depcrate_core_config_configread_file_by_commit {
() => {
// Module: crate::core::config::config
// Provides: {"read_file_by_commit"}
// Dependencies: {}
# [doc = " Returns the content of the given file at a specific commit."] pub (crate) fn read_file_by_commit < 'a > (dwn_ctx : impl AsRef < DownloadContext < 'a > > , rust_info : & channel :: GitInfo , file : & Path , commit : & str ,) -> String { let dwn_ctx = dwn_ctx . as_ref () ; assert ! (rust_info . is_managed_git_subrepository () , "`Config::read_file_by_commit` is not supported in non-git sources.") ; let mut git = helpers :: git (Some (dwn_ctx . src)) ; git . arg ("show") . arg (format ! ("{commit}:{}" , file . to_str () . unwrap ())) ; git . run_capture_stdout (dwn_ctx . exec_ctx) . stdout () }
};
}
