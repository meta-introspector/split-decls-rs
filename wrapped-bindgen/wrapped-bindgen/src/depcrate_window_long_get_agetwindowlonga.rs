// Generated macro for GetWindowLongA (function)
macro_rules! Depcrate_window_long_get_aGetWindowLongA {
() => {
// Module: crate::window_long_get_a
// Provides: {"GetWindowLongA"}
// Dependencies: {}
# [inline] pub unsafe fn GetWindowLongA (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX) -> i32 { windows_core :: link ! ("user32.dll" "system" fn GetWindowLongA (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX) -> i32) ; unsafe { GetWindowLongA (hwnd , nindex) } }
};
}
