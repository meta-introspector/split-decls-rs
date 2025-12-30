// Generated macro for stdout_handle (function)
macro_rules! Depcrate_processstdout_handle {
() => {
// Module: crate::process
// Provides: {"stdout_handle"}
// Dependencies: {}
fn stdout_handle () -> win :: Result < HANDLE > { let conout : Vec < u16 > = convert_osstr_to_utf16 (OsStr :: new ("CONOUT$")) ; let conout = PCWSTR (conout . as_ptr ()) ; unsafe { CreateFileW (conout , (FILE_GENERIC_READ | FILE_GENERIC_WRITE) . 0 , FILE_SHARE_READ | FILE_SHARE_WRITE , None , OPEN_EXISTING , FILE_ATTRIBUTE_NORMAL , HANDLE :: default () ,) } }
};
}
