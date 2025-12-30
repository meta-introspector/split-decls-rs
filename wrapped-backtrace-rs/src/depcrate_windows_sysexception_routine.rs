// Generated macro for EXCEPTION_ROUTINE (type)
macro_rules! Depcrate_windows_sysEXCEPTION_ROUTINE {
() => {
// Module: crate::windows_sys
// Provides: {"EXCEPTION_ROUTINE"}
// Dependencies: {}
pub type EXCEPTION_ROUTINE = Option < unsafe extern "system" fn (exceptionrecord : * mut EXCEPTION_RECORD , establisherframe : * const core :: ffi :: c_void , contextrecord : * mut CONTEXT , dispatchercontext : * const core :: ffi :: c_void ,) -> EXCEPTION_DISPOSITION , > ;
};
}
