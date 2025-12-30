// Generated macro for SetWindowLongPtrW (function)
macro_rules! Depcrate_window_long_set_wSetWindowLongPtrW {
() => {
// Module: crate::window_long_set_w
// Provides: {"SetWindowLongPtrW"}
// Dependencies: {}
# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] # [inline] pub unsafe fn SetWindowLongPtrW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : isize ,) -> isize { windows_core :: link ! ("user32.dll" "system" fn SetWindowLongPtrW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : isize) -> isize) ; unsafe { SetWindowLongPtrW (hwnd , nindex , dwnewlong) } }
};
}
