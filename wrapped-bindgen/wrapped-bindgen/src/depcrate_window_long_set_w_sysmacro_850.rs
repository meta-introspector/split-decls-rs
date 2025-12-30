// Generated macro for macro_850 (macro)
macro_rules! Depcrate_window_long_set_w_sysmacro_850 {
() => {
// Module: crate::window_long_set_w_sys
// Provides: {"macro_850"}
// Dependencies: {}
windows_link :: link ! ("user32.dll" "system" fn SetWindowLongW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : i32) -> i32) ;
};
}
