// Generated macro for PREAD_PROCESS_MEMORY_ROUTINE64 (type)
macro_rules! Depcrate_windows_sysPREAD_PROCESS_MEMORY_ROUTINE64 {
() => {
// Module: crate::windows_sys
// Provides: {"PREAD_PROCESS_MEMORY_ROUTINE64"}
// Dependencies: {}
pub type PREAD_PROCESS_MEMORY_ROUTINE64 = Option < unsafe extern "system" fn (hprocess : HANDLE , qwbaseaddress : u64 , lpbuffer : * mut core :: ffi :: c_void , nsize : u32 , lpnumberofbytesread : * mut u32 ,) -> BOOL , > ;
};
}
