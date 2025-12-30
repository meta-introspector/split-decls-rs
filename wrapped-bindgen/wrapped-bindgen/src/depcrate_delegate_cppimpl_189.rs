// Generated macro for impl_189 (impl)
macro_rules! Depcrate_delegate_cppimpl_189 {
() => {
// Module: crate::delegate_cpp
// Provides: {"impl_189"}
// Dependencies: {}
impl windows_core :: Free for HANDLE { # [inline] unsafe fn free (& mut self) { if ! self . is_invalid () { windows_core :: link ! ("kernel32.dll" "system" fn CloseHandle (hobject : * mut core :: ffi :: c_void) -> i32) ; unsafe { CloseHandle (self . 0) ; } } } }
};
}
