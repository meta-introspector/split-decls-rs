// Generated macro for ShellExecuteExW (function)
macro_rules! Depcrate_windowsShellExecuteExW {
() => {
// Module: crate::windows
// Provides: {"ShellExecuteExW"}
// Dependencies: {}
# [doc = " Performs an operation on a specified file."] # [doc = ""] # [doc = " <https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecuteexw>"] # [allow (non_snake_case)] # [cfg (feature = "shellexecute-on-windows")] pub unsafe fn ShellExecuteExW (info : * mut ffi :: SHELLEXECUTEINFOW) -> std :: io :: Result < () > { if ffi :: ShellExecuteExW (info) == 1 { Ok (()) } else { Err (std :: io :: Error :: last_os_error ()) } }
};
}
