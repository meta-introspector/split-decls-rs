// Generated macro for UNWIND_HISTORY_TABLE_ENTRY (struct)
macro_rules! Depcrate_windows_sysUNWIND_HISTORY_TABLE_ENTRY {
() => {
// Module: crate::windows_sys
// Provides: {"UNWIND_HISTORY_TABLE_ENTRY"}
// Dependencies: {}
# [repr (C)] # [cfg (any (target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub struct UNWIND_HISTORY_TABLE_ENTRY { pub ImageBase : usize , pub FunctionEntry : * mut IMAGE_RUNTIME_FUNCTION_ENTRY , }
};
}
