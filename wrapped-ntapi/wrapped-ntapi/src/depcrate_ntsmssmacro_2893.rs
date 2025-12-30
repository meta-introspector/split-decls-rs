// Generated macro for macro_2893 (macro)
macro_rules! Depcrate_ntsmssmacro_2893 {
() => {
// Module: crate::ntsmss
// Provides: {"macro_2893"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlConnectToSm (ApiPortName : PUNICODE_STRING , ApiPortHandle : HANDLE , ProcessImageType : DWORD , SmssConnection : PHANDLE ,) -> NTSTATUS ; fn RtlSendMsgToSm (ApiPortHandle : HANDLE , MessageData : PPORT_MESSAGE ,) -> NTSTATUS ; } }
};
}
