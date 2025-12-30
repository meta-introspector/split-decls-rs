// Generated macro for imp (module)
macro_rules! Depcrate_process_builderimp {
() => {
// Module: crate::process_builder
// Provides: {"imp"}
// Dependencies: {}
# [cfg (windows)] mod imp { use super :: { ProcessBuilder , ProcessError } ; use anyhow :: Result ; use std :: io ; use windows_sys :: Win32 :: Foundation :: { FALSE , TRUE } ; use windows_sys :: Win32 :: System :: Console :: SetConsoleCtrlHandler ; use windows_sys :: core :: BOOL ; unsafe extern "system" fn ctrlc_handler (_ : u32) -> BOOL { TRUE } pub fn exec_replace (process_builder : & ProcessBuilder) -> Result < () > { unsafe { if SetConsoleCtrlHandler (Some (ctrlc_handler) , TRUE) == FALSE { return Err (ProcessError :: new ("Could not set Ctrl-C handler." , None , None) . into ()) ; } } process_builder . exec () } pub fn command_line_too_big (err : & io :: Error) -> bool { use windows_sys :: Win32 :: Foundation :: ERROR_FILENAME_EXCED_RANGE ; err . raw_os_error () == Some (ERROR_FILENAME_EXCED_RANGE as i32) } }
};
}
