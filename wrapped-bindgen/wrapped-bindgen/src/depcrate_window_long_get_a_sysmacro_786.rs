// Generated macro for macro_786 (macro)
macro_rules! Depcrate_window_long_get_a_sysmacro_786 {
() => {
// Module: crate::window_long_get_a_sys
// Provides: {"macro_786"}
// Dependencies: {}
# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] windows_link :: link ! ("user32.dll" "system" fn GetWindowLongPtrA (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX) -> isize) ;
};
}
