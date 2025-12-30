// Generated macro for impl_17 (impl)
macro_rules! Depcrate_bool_event_sans_referenceimpl_17 {
() => {
// Module: crate::bool_event_sans_reference
// Provides: {"impl_17"}
// Dependencies: {}
impl windows_core :: Free for HANDLE { # [inline] unsafe fn free (& mut self) { if ! self . is_invalid () { windows_core :: link ! ("kernel32.dll" "system" fn CloseHandle (hobject : * mut core :: ffi :: c_void) -> i32) ; unsafe { CloseHandle (self . 0) ; } } } }
};
}
