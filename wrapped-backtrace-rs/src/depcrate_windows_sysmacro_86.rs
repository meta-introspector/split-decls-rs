// Generated macro for macro_86 (macro)
macro_rules! Depcrate_windows_sysmacro_86 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_86"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn StackWalkEx (machinetype : u32 , hprocess : HANDLE , hthread : HANDLE , stackframe : * mut STACKFRAME_EX , contextrecord : * mut core :: ffi :: c_void , readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64 , functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64 , getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64 , translateaddress : PTRANSLATE_ADDRESS_ROUTINE64 , flags : u32) -> BOOL) ;
};
}
