// Generated macro for WaitForSingleObjectEx (function)
macro_rules! Depcrate_bool_event_sans_referenceWaitForSingleObjectEx {
() => {
// Module: crate::bool_event_sans_reference
// Provides: {"WaitForSingleObjectEx"}
// Dependencies: {}
# [inline] pub unsafe fn WaitForSingleObjectEx (hhandle : HANDLE , dwmilliseconds : u32 , balertable : bool ,) -> WAIT_EVENT { windows_core :: link ! ("kernel32.dll" "system" fn WaitForSingleObjectEx (hhandle : HANDLE , dwmilliseconds : u32 , balertable : windows_core :: BOOL) -> WAIT_EVENT) ; unsafe { WaitForSingleObjectEx (hhandle , dwmilliseconds , balertable . into ()) } }
};
}
