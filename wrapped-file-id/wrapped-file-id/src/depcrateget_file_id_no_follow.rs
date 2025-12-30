// Generated macro for get_file_id_no_follow (function)
macro_rules! Depcrateget_file_id_no_follow {
() => {
// Module: crate
// Provides: {"get_file_id_no_follow"}
// Dependencies: {}
# [doc = " Get the `FileId` for the file or directory at `path` without following symlinks/reparse points"] # [cfg (target_family = "windows")] pub fn get_file_id_no_follow (path : impl AsRef < Path >) -> io :: Result < FileId > { let file = open_file_no_follow (path) ? ; unsafe { get_file_info_ex (& file) . or_else (| _ | get_file_info (& file)) } }
};
}
