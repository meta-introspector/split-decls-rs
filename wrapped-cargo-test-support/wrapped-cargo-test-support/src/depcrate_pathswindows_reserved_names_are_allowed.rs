// Generated macro for windows_reserved_names_are_allowed (function)
macro_rules! Depcrate_pathswindows_reserved_names_are_allowed {
() => {
// Module: crate::paths
// Provides: {"windows_reserved_names_are_allowed"}
// Dependencies: {}
# [doc = " Returns true if names such as aux.* are allowed."] # [doc = ""] # [doc = " Traditionally, Windows did not allow a set of file names (see `is_windows_reserved`"] # [doc = " for a list). More recent versions of Windows have relaxed this restriction. This test"] # [doc = " determines whether we are running in a mode that allows Windows reserved names."] # [cfg (windows)] pub fn windows_reserved_names_are_allowed () -> bool { use std :: ffi :: OsStr ; use std :: os :: windows :: ffi :: OsStrExt ; use std :: ptr ; use windows_sys :: Win32 :: Storage :: FileSystem :: GetFullPathNameW ; let test_file_name : Vec < _ > = OsStr :: new ("aux.rs") . encode_wide () . chain ([0]) . collect () ; let buffer_length = unsafe { GetFullPathNameW (test_file_name . as_ptr () , 0 , ptr :: null_mut () , ptr :: null_mut ()) } ; if buffer_length == 0 { return false ; } let mut buffer = vec ! [0u16 ; buffer_length as usize] ; let result = unsafe { GetFullPathNameW (test_file_name . as_ptr () , buffer_length , buffer . as_mut_ptr () , ptr :: null_mut () ,) } ; if result == 0 { return false ; } let prefix : Vec < _ > = OsStr :: new ("\\\\.\\") . encode_wide () . collect () ; if buffer . starts_with (& prefix) { false } else { true } }
};
}
