// Generated macro for macro_38 (macro)
macro_rules! Depcrate_windows_sysmacro_38 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_38"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn PeekNamedPipe (hnamedpipe : HANDLE , lpbuffer : * mut core :: ffi :: c_void , nbuffersize : u32 , lpbytesread : * mut u32 , lptotalbytesavail : * mut u32 , lpbytesleftthismessage : * mut u32) -> BOOL) ;
};
}
