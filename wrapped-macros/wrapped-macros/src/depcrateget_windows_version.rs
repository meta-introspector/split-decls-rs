// Generated macro for get_windows_version (function)
macro_rules! Depcrateget_windows_version {
() => {
// Module: crate
// Provides: {"get_windows_version"}
// Dependencies: {}
fn get_windows_version () -> Result < OSVERSIONINFOW > { let mut version_info = MaybeUninit :: uninit () ; let result = unsafe { RtlGetVersion (version_info . as_mut_ptr ()) } ; if result == STATUS_SUCCESS { Ok (unsafe { version_info . assume_init () }) } else { let error = io :: Error :: last_os_error () ; Err (Error :: new (Span :: call_site () , format ! ("failed syscall: {}" , error) ,)) } }
};
}
