// Generated macro for IsCharLowerA (function)
macro_rules! Depcrate_bindingsIsCharLowerA {
() => {
// Module: crate::bindings
// Provides: {"IsCharLowerA"}
// Dependencies: {}
# [inline] pub unsafe fn IsCharLowerA (ch : i8) -> windows_core :: Result < () > { windows_core :: link ! ("user32.dll" "system" fn IsCharLowerA (ch : i8) -> windows_core :: BOOL) ; unsafe { IsCharLowerA (ch) . ok () } }
};
}
