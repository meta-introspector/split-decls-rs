// Generated macro for macro_84 (macro)
macro_rules! Depcrate_windows_sysmacro_84 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_84"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn EnumerateLoadedModulesW64 (hprocess : HANDLE , enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64 , usercontext : * const core :: ffi :: c_void) -> BOOL) ;
};
}
