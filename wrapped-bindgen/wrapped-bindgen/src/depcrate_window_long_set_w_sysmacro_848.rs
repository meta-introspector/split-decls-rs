// Generated macro for macro_848 (macro)
macro_rules! Depcrate_window_long_set_w_sysmacro_848 {
() => {
// Module: crate::window_long_set_w_sys
// Provides: {"macro_848"}
// Dependencies: {}
# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] windows_link :: link ! ("user32.dll" "system" fn SetWindowLongPtrW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : isize) -> isize) ;
};
}
