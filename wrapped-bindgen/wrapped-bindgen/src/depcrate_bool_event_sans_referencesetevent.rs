// Generated macro for SetEvent (function)
macro_rules! Depcrate_bool_event_sans_referenceSetEvent {
() => {
// Module: crate::bool_event_sans_reference
// Provides: {"SetEvent"}
// Dependencies: {}
# [inline] pub unsafe fn SetEvent (hevent : HANDLE) -> windows_core :: Result < () > { windows_core :: link ! ("kernel32.dll" "system" fn SetEvent (hevent : HANDLE) -> windows_core :: BOOL) ; unsafe { SetEvent (hevent) . ok () } }
};
}
