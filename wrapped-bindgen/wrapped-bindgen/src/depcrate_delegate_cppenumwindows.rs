// Generated macro for EnumWindows (function)
macro_rules! Depcrate_delegate_cppEnumWindows {
() => {
// Module: crate::delegate_cpp
// Provides: {"EnumWindows"}
// Dependencies: {}
# [inline] pub unsafe fn EnumWindows (lpenumfunc : WNDENUMPROC , lparam : LPARAM) -> windows_core :: Result < () > { windows_core :: link ! ("user32.dll" "system" fn EnumWindows (lpenumfunc : WNDENUMPROC , lparam : LPARAM) -> windows_core :: BOOL) ; unsafe { EnumWindows (lpenumfunc , lparam) . ok () } }
};
}
