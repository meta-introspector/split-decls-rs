// Generated macro for GlobalMemoryStatus (function)
macro_rules! Depcrate_fn_return_void_winGlobalMemoryStatus {
() => {
// Module: crate::fn_return_void_win
// Provides: {"GlobalMemoryStatus"}
// Dependencies: {}
# [inline] pub unsafe fn GlobalMemoryStatus (lpbuffer : * mut MEMORYSTATUS) { windows_core :: link ! ("kernel32.dll" "system" fn GlobalMemoryStatus (lpbuffer : * mut MEMORYSTATUS)) ; unsafe { GlobalMemoryStatus (lpbuffer as _) } }
};
}
