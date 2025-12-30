// Generated macro for EnableMouseInPointer (function)
macro_rules! Depcrate_boolEnableMouseInPointer {
() => {
// Module: crate::bool
// Provides: {"EnableMouseInPointer"}
// Dependencies: {}
# [inline] pub unsafe fn EnableMouseInPointer (fenable : bool) -> windows_core :: Result < () > { windows_core :: link ! ("user32.dll" "system" fn EnableMouseInPointer (fenable : windows_core :: BOOL) -> windows_core :: BOOL) ; unsafe { EnableMouseInPointer (fenable . into ()) . ok () } }
};
}
