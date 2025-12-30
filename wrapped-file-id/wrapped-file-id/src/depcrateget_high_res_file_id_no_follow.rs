// Generated macro for get_high_res_file_id_no_follow (function)
macro_rules! Depcrateget_high_res_file_id_no_follow {
() => {
// Module: crate
// Provides: {"get_high_res_file_id_no_follow"}
// Dependencies: {}
# [doc = " Get the `FileId` with the high resolution variant for the file or directory at `path` without following symlinks/reparse points"] # [cfg (target_family = "windows")] pub fn get_high_res_file_id_no_follow (path : impl AsRef < Path >) -> io :: Result < FileId > { let file = open_file_no_follow (path) ? ; unsafe { get_file_info_ex (& file) } }
};
}
