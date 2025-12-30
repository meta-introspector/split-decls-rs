// Generated macro for macro_117 (macro)
macro_rules! Depcrate_windows_sysmacro_117 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_117"}
// Dependencies: {}
# [cfg (any (target_arch = "arm64ec" , target_arch = "x86_64"))] windows_link :: link ! ("kernel32.dll" "system" fn RtlVirtualUnwind (handlertype : RTL_VIRTUAL_UNWIND_HANDLER_TYPE , imagebase : u64 , controlpc : u64 , functionentry : * const IMAGE_RUNTIME_FUNCTION_ENTRY , contextrecord : * mut CONTEXT , handlerdata : * mut * mut core :: ffi :: c_void , establisherframe : * mut u64 , contextpointers : * mut KNONVOLATILE_CONTEXT_POINTERS) -> EXCEPTION_ROUTINE) ;
};
}
