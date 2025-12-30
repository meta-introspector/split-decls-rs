// Generated macro for macro_85 (macro)
macro_rules! Depcrate_windows_sysmacro_85 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_85"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn StackWalk64 (machinetype : u32 , hprocess : HANDLE , hthread : HANDLE , stackframe : * mut STACKFRAME64 , contextrecord : * mut core :: ffi :: c_void , readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64 , functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64 , getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64 , translateaddress : PTRANSLATE_ADDRESS_ROUTINE64) -> BOOL) ;
};
}
