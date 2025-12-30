// Generated macro for macro_116 (macro)
macro_rules! Depcrate_windows_sysmacro_116 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_116"}
// Dependencies: {}
# [cfg (target_arch = "aarch64")] windows_link :: link ! ("kernel32.dll" "system" fn RtlVirtualUnwind (handlertype : RTL_VIRTUAL_UNWIND_HANDLER_TYPE , imagebase : usize , controlpc : usize , functionentry : * const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY , contextrecord : * mut CONTEXT , handlerdata : * mut * mut core :: ffi :: c_void , establisherframe : * mut usize , contextpointers : * mut KNONVOLATILE_CONTEXT_POINTERS) -> EXCEPTION_ROUTINE) ;
};
}
