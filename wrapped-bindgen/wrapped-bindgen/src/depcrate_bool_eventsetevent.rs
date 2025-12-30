// Generated macro for SetEvent (function)
macro_rules! Depcrate_bool_eventSetEvent {
() => {
// Module: crate::bool_event
// Provides: {"SetEvent"}
// Dependencies: {}
# [inline] pub unsafe fn SetEvent (hevent : windows :: Win32 :: Foundation :: HANDLE) -> windows_core :: Result < () > { windows_core :: link ! ("kernel32.dll" "system" fn SetEvent (hevent : windows :: Win32 :: Foundation :: HANDLE) -> windows_core :: BOOL) ; unsafe { SetEvent (hevent) . ok () } }
};
}
