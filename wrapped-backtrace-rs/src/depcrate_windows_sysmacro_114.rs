// Generated macro for macro_114 (macro)
macro_rules! Depcrate_windows_sysmacro_114 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_114"}
// Dependencies: {}
# [cfg (target_arch = "aarch64")] windows_link :: link ! ("kernel32.dll" "system" fn RtlLookupFunctionEntry (controlpc : usize , imagebase : * mut usize , historytable : * mut UNWIND_HISTORY_TABLE) -> * mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY) ;
};
}
