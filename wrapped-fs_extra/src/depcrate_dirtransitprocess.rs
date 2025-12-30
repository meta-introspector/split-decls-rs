// Generated macro for TransitProcess (struct)
macro_rules! Depcrate_dirTransitProcess {
() => {
// Module: crate::dir
// Provides: {"TransitProcess"}
// Dependencies: {}
# [doc = " A structure which include information about the current status of the copy or move directory."] pub struct TransitProcess { # [doc = " Copied bytes on this time for folder"] pub copied_bytes : u64 , # [doc = " All the bytes which should to copy or move (dir size)."] pub total_bytes : u64 , # [doc = " Copied bytes on this time for file."] pub file_bytes_copied : u64 , # [doc = " Size current copied file."] pub file_total_bytes : u64 , # [doc = " Name current copied file."] pub file_name : String , # [doc = " Transit state"] pub state : TransitState , }
};
}
