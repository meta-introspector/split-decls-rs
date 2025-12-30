// Generated macro for get_high_res_file_id (function)
macro_rules! Depcrateget_high_res_file_id {
() => {
// Module: crate
// Provides: {"get_high_res_file_id"}
// Dependencies: {}
# [doc = " Get the `FileId` with the high resolution variant for the file or directory at `path`"] # [cfg (target_family = "windows")] pub fn get_high_res_file_id (path : impl AsRef < Path >) -> io :: Result < FileId > { let file = open_file (path) ? ; unsafe { get_file_info_ex (& file) } }
};
}
