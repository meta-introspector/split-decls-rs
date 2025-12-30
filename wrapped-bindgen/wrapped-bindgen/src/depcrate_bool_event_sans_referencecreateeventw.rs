// Generated macro for CreateEventW (function)
macro_rules! Depcrate_bool_event_sans_referenceCreateEventW {
() => {
// Module: crate::bool_event_sans_reference
// Provides: {"CreateEventW"}
// Dependencies: {}
# [inline] pub unsafe fn CreateEventW < P3 > (lpeventattributes : Option < * const SECURITY_ATTRIBUTES > , bmanualreset : bool , binitialstate : bool , lpname : P3 ,) -> windows_core :: Result < HANDLE > where P3 : windows_core :: Param < windows_core :: PCWSTR > , { windows_core :: link ! ("kernel32.dll" "system" fn CreateEventW (lpeventattributes : * const SECURITY_ATTRIBUTES , bmanualreset : windows_core :: BOOL , binitialstate : windows_core :: BOOL , lpname : windows_core :: PCWSTR) -> HANDLE) ; let result__ = unsafe { CreateEventW (lpeventattributes . unwrap_or (core :: mem :: zeroed ()) as _ , bmanualreset . into () , binitialstate . into () , lpname . param () . abi () ,) } ; (! result__ . is_invalid ()) . then_some (result__) . ok_or_else (windows_core :: Error :: from_thread) }
};
}
