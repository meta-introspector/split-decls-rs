// Generated macro for impl_199 (impl)
macro_rules! Depcrate_delegate_cppimpl_199 {
() => {
// Module: crate::delegate_cpp
// Provides: {"impl_199"}
// Dependencies: {}
impl windows_core :: Free for HMODULE { # [inline] unsafe fn free (& mut self) { if ! self . is_invalid () { windows_core :: link ! ("kernel32.dll" "system" fn FreeLibrary (hlibmodule : * mut core :: ffi :: c_void) -> i32) ; unsafe { FreeLibrary (self . 0) ; } } } }
};
}
