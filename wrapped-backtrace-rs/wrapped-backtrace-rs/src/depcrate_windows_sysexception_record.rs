// Generated macro for EXCEPTION_RECORD (struct)
macro_rules! Depcrate_windows_sysEXCEPTION_RECORD {
() => {
// Module: crate::windows_sys
// Provides: {"EXCEPTION_RECORD"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct EXCEPTION_RECORD { pub ExceptionCode : NTSTATUS , pub ExceptionFlags : u32 , pub ExceptionRecord : * mut EXCEPTION_RECORD , pub ExceptionAddress : * mut core :: ffi :: c_void , pub NumberParameters : u32 , pub ExceptionInformation : [usize ; 15] , }
};
}
