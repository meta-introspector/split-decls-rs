// Generated macro for get_file_id (function)
macro_rules! Depcrateget_file_id {
() => {
// Module: crate
// Provides: {"get_file_id"}
// Dependencies: {}
# [doc = " Get the `FileId` for the file or directory at `path`"] # [cfg (target_family = "windows")] pub fn get_file_id (path : impl AsRef < Path >) -> io :: Result < FileId > { let file = open_file (path) ? ; unsafe { get_file_info_ex (& file) . or_else (| _ | get_file_info (& file)) } }
};
}
