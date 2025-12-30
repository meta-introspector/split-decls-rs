// Generated macro for macro_115 (macro)
macro_rules! Depcrate_windows_sysmacro_115 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_115"}
// Dependencies: {}
# [cfg (any (target_arch = "arm64ec" , target_arch = "x86_64"))] windows_link :: link ! ("kernel32.dll" "system" fn RtlLookupFunctionEntry (controlpc : u64 , imagebase : * mut u64 , historytable : * mut UNWIND_HISTORY_TABLE) -> * mut IMAGE_RUNTIME_FUNCTION_ENTRY) ;
};
}
