// Generated macro for impl_776 (impl)
macro_rules! Depcrate_window_long_get_aimpl_776 {
() => {
// Module: crate::window_long_get_a
// Provides: {"impl_776"}
// Dependencies: {}
impl windows_core :: Free for HANDLE { # [inline] unsafe fn free (& mut self) { if ! self . is_invalid () { windows_core :: link ! ("kernel32.dll" "system" fn CloseHandle (hobject : * mut core :: ffi :: c_void) -> i32) ; unsafe { CloseHandle (self . 0) ; } } } }
};
}
