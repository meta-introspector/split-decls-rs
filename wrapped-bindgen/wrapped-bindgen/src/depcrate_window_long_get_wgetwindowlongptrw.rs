// Generated macro for GetWindowLongPtrW (function)
macro_rules! Depcrate_window_long_get_wGetWindowLongPtrW {
() => {
// Module: crate::window_long_get_w
// Provides: {"GetWindowLongPtrW"}
// Dependencies: {}
# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] # [inline] pub unsafe fn GetWindowLongPtrW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX) -> isize { windows_core :: link ! ("user32.dll" "system" fn GetWindowLongPtrW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX) -> isize) ; unsafe { GetWindowLongPtrW (hwnd , nindex) } }
};
}
