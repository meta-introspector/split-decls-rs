// Generated macro for impl_797 (impl)
macro_rules! Depcrate_window_long_get_wimpl_797 {
() => {
// Module: crate::window_long_get_w
// Provides: {"impl_797"}
// Dependencies: {}
impl windows_core :: Free for HANDLE { # [inline] unsafe fn free (& mut self) { if ! self . is_invalid () { windows_core :: link ! ("kernel32.dll" "system" fn CloseHandle (hobject : * mut core :: ffi :: c_void) -> i32) ; unsafe { CloseHandle (self . 0) ; } } } }
};
}
