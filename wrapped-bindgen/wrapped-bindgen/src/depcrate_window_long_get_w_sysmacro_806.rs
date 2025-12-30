// Generated macro for macro_806 (macro)
macro_rules! Depcrate_window_long_get_w_sysmacro_806 {
() => {
// Module: crate::window_long_get_w_sys
// Provides: {"macro_806"}
// Dependencies: {}
# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] windows_link :: link ! ("user32.dll" "system" fn GetWindowLongPtrW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX) -> isize) ;
};
}
