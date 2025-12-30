// Generated macro for GetProcAddress (function)
macro_rules! Depcrate_delegate_cppGetProcAddress {
() => {
// Module: crate::delegate_cpp
// Provides: {"GetProcAddress"}
// Dependencies: {}
# [inline] pub unsafe fn GetProcAddress < P1 > (hmodule : HMODULE , lpprocname : P1) -> FARPROC where P1 : windows_core :: Param < windows_core :: PCSTR > , { windows_core :: link ! ("kernel32.dll" "system" fn GetProcAddress (hmodule : HMODULE , lpprocname : windows_core :: PCSTR) -> FARPROC) ; unsafe { GetProcAddress (hmodule , lpprocname . param () . abi ()) } }
};
}
