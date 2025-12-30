// Generated macro for get_low_res_file_id (function)
macro_rules! Depcrateget_low_res_file_id {
() => {
// Module: crate
// Provides: {"get_low_res_file_id"}
// Dependencies: {}
# [doc = " Get the `FileId` with the low resolution variant for the file or directory at `path`"] # [cfg (target_family = "windows")] pub fn get_low_res_file_id (path : impl AsRef < Path >) -> io :: Result < FileId > { let file = open_file (path) ? ; unsafe { get_file_info (& file) } }
};
}
