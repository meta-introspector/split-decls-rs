// Generated macro for PENUMLOADED_MODULES_CALLBACKW64 (type)
macro_rules! Depcrate_windows_sysPENUMLOADED_MODULES_CALLBACKW64 {
() => {
// Module: crate::windows_sys
// Provides: {"PENUMLOADED_MODULES_CALLBACKW64"}
// Dependencies: {}
pub type PENUMLOADED_MODULES_CALLBACKW64 = Option < unsafe extern "system" fn (modulename : PCWSTR , modulebase : u64 , modulesize : u32 , usercontext : * const core :: ffi :: c_void ,) -> BOOL , > ;
};
}
