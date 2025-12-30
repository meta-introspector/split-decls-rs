// Generated macro for macro_828 (macro)
macro_rules! Depcrate_window_long_set_a_sysmacro_828 {
() => {
// Module: crate::window_long_set_a_sys
// Provides: {"macro_828"}
// Dependencies: {}
# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] windows_link :: link ! ("user32.dll" "system" fn SetWindowLongPtrA (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : isize) -> isize) ;
};
}
