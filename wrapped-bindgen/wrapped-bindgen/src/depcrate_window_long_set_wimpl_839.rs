// Generated macro for impl_839 (impl)
macro_rules! Depcrate_window_long_set_wimpl_839 {
() => {
// Module: crate::window_long_set_w
// Provides: {"impl_839"}
// Dependencies: {}
impl windows_core :: Free for HANDLE { # [inline] unsafe fn free (& mut self) { if ! self . is_invalid () { windows_core :: link ! ("kernel32.dll" "system" fn CloseHandle (hobject : * mut core :: ffi :: c_void) -> i32) ; unsafe { CloseHandle (self . 0) ; } } } }
};
}
