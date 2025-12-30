// Generated macro for SetWindowLongPtrA (function)
macro_rules! Depcrate_window_long_set_aSetWindowLongPtrA {
() => {
// Module: crate::window_long_set_a
// Provides: {"SetWindowLongPtrA"}
// Dependencies: {}
# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] # [inline] pub unsafe fn SetWindowLongPtrA (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : isize ,) -> isize { windows_core :: link ! ("user32.dll" "system" fn SetWindowLongPtrA (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX , dwnewlong : isize) -> isize) ; unsafe { SetWindowLongPtrA (hwnd , nindex , dwnewlong) } }
};
}
