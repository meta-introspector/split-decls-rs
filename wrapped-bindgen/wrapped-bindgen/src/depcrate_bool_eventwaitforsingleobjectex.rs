// Generated macro for WaitForSingleObjectEx (function)
macro_rules! Depcrate_bool_eventWaitForSingleObjectEx {
() => {
// Module: crate::bool_event
// Provides: {"WaitForSingleObjectEx"}
// Dependencies: {}
# [inline] pub unsafe fn WaitForSingleObjectEx (hhandle : windows :: Win32 :: Foundation :: HANDLE , dwmilliseconds : u32 , balertable : bool ,) -> windows :: Win32 :: Foundation :: WAIT_EVENT { windows_core :: link ! ("kernel32.dll" "system" fn WaitForSingleObjectEx (hhandle : windows :: Win32 :: Foundation :: HANDLE , dwmilliseconds : u32 , balertable : windows_core :: BOOL) -> windows :: Win32 :: Foundation :: WAIT_EVENT) ; unsafe { WaitForSingleObjectEx (hhandle , dwmilliseconds , balertable . into ()) } }
};
}
