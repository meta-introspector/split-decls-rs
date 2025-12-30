// Generated macro for STACKFRAME64 (struct)
macro_rules! Depcrate_windows_sysSTACKFRAME64 {
() => {
// Module: crate::windows_sys
// Provides: {"STACKFRAME64"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct STACKFRAME64 { pub AddrPC : ADDRESS64 , pub AddrReturn : ADDRESS64 , pub AddrFrame : ADDRESS64 , pub AddrStack : ADDRESS64 , pub AddrBStore : ADDRESS64 , pub FuncTableEntry : * mut core :: ffi :: c_void , pub Params : [u64 ; 4] , pub Far : BOOL , pub Virtual : BOOL , pub Reserved : [u64 ; 3] , pub KdHelp : KDHELP64 , }
};
}
