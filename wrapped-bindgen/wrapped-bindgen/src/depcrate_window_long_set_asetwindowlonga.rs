// Generated macro for SetWindowLongA (function)
macro_rules! Depcrate_window_long_set_aSetWindowLongA {
() => {
// Module: crate::window_long_set_a
// Provides: {"SetWindowLongA"}
// Dependencies: {}
# [inline] pub unsafe fn SetWindowLongA (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : i32) -> i32 { windows_core :: link ! ("user32.dll" "system" fn SetWindowLongA (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : i32) -> i32) ; unsafe { SetWindowLongA (hwnd , nindex , dwnewlong) } }
};
}
