// Generated macro for GetWindowLongPtrA (function)
macro_rules! Depcrate_window_long_get_aGetWindowLongPtrA {
() => {
// Module: crate::window_long_get_a
// Provides: {"GetWindowLongPtrA"}
// Dependencies: {}
# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] # [inline] pub unsafe fn GetWindowLongPtrA (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX) -> isize { windows_core :: link ! ("user32.dll" "system" fn GetWindowLongPtrA (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX) -> isize) ; unsafe { GetWindowLongPtrA (hwnd , nindex) } }
};
}
