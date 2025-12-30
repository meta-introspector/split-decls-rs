// Generated macro for GetWindowLongW (function)
macro_rules! Depcrate_window_long_get_wGetWindowLongW {
() => {
// Module: crate::window_long_get_w
// Provides: {"GetWindowLongW"}
// Dependencies: {}
# [inline] pub unsafe fn GetWindowLongW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX) -> i32 { windows_core :: link ! ("user32.dll" "system" fn GetWindowLongW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX) -> i32) ; unsafe { GetWindowLongW (hwnd , nindex) } }
};
}
