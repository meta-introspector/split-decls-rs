// Generated macro for GetTickCount (function)
macro_rules! Depcrate_fn_winGetTickCount {
() => {
// Module: crate::fn_win
// Provides: {"GetTickCount"}
// Dependencies: {}
# [inline] pub unsafe fn GetTickCount () -> u32 { windows_core :: link ! ("kernel32.dll" "system" fn GetTickCount () -> u32) ; unsafe { GetTickCount () } }
};
}
