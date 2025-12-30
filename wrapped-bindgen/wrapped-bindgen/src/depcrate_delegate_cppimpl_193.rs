// Generated macro for impl_193 (impl)
macro_rules! Depcrate_delegate_cppimpl_193 {
() => {
// Module: crate::delegate_cpp
// Provides: {"impl_193"}
// Dependencies: {}
impl windows_core :: Free for HINSTANCE { # [inline] unsafe fn free (& mut self) { if ! self . is_invalid () { windows_core :: link ! ("kernel32.dll" "system" fn FreeLibrary (hlibmodule : * mut core :: ffi :: c_void) -> i32) ; unsafe { FreeLibrary (self . 0) ; } } } }
};
}
