// Generated macro for macro_1255 (macro)
macro_rules! Depcrate_ntlpcapimacro_1255 {
() => {
// Module: crate::ntlpcapi
// Provides: {"macro_1255"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtQueryInformationPort (PortHandle : HANDLE , PortInformationClass : PORT_INFORMATION_CLASS , PortInformation : PVOID , Length : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; } }
};
}
