// Generated macro for component_is_windows_device (function)
macro_rules! Depcrate_pathcomponent_is_windows_device {
() => {
// Module: crate::path
// Provides: {"component_is_windows_device"}
// Dependencies: {}
# [doc = " Return `true` if the path component at `input` looks like a Windows device, like `CON`"] # [doc = " or `LPT1` (case-insensitively)."] # [doc = ""] # [doc = " This is relevant only on Windows, where one may be tricked into reading or writing to such devices."] # [doc = " When reading from `CON`, a console-program may block until the user provided input."] pub fn component_is_windows_device (input : & BStr) -> bool { is_win_device (input) }
};
}
