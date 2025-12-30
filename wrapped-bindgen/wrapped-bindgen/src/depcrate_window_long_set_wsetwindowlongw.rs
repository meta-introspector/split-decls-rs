// Generated macro for SetWindowLongW (function)
macro_rules! Depcrate_window_long_set_wSetWindowLongW {
() => {
// Module: crate::window_long_set_w
// Provides: {"SetWindowLongW"}
// Dependencies: {}
# [inline] pub unsafe fn SetWindowLongW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : i32) -> i32 { windows_core :: link ! ("user32.dll" "system" fn SetWindowLongW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : i32) -> i32) ; unsafe { SetWindowLongW (hwnd , nindex , dwnewlong) } }
};
}
