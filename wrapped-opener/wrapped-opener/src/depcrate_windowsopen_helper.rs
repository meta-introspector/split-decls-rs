// Generated macro for open_helper (function)
macro_rules! Depcrate_windowsopen_helper {
() => {
// Module: crate::windows
// Provides: {"open_helper"}
// Dependencies: {}
pub (crate) fn open_helper (path : & OsStr) -> Result < () , OpenError > { let path = convert_path (path) . map_err (OpenError :: Io) ? ; let operation : Vec < u16 > = OsStr :: new ("open\0") . encode_wide () . collect () ; let result = unsafe { ShellExecuteW (ptr :: null_mut () , operation . as_ptr () , path . as_ptr () , ptr :: null () , ptr :: null () , SW_SHOW ,) } ; if result as usize as isize > 32 { Ok (()) } else { Err (OpenError :: Io (io :: Error :: last_os_error ())) } }
};
}
