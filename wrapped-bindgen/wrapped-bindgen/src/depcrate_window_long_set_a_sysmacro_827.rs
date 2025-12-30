// Generated macro for macro_827 (macro)
macro_rules! Depcrate_window_long_set_a_sysmacro_827 {
() => {
// Module: crate::window_long_set_a_sys
// Provides: {"macro_827"}
// Dependencies: {}
windows_link :: link ! ("user32.dll" "system" fn SetWindowLongA (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : i32) -> i32) ;
};
}
