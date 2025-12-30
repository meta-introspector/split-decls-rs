// Generated macro for macro_1315 (macro)
macro_rules! Depcrate_ntlpcapimacro_1315 {
() => {
// Module: crate::ntlpcapi
// Provides: {"macro_1315"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtAlpcConnectPort (PortHandle : PHANDLE , PortName : PUNICODE_STRING , ObjectAttributes : POBJECT_ATTRIBUTES , PortAttributes : PALPC_PORT_ATTRIBUTES , Flags : ULONG , RequiredServerSid : PSID , ConnectionMessage : PPORT_MESSAGE , BufferLength : PULONG , OutMessageAttributes : PALPC_MESSAGE_ATTRIBUTES , InMessageAttributes : PALPC_MESSAGE_ATTRIBUTES , Timeout : PLARGE_INTEGER ,) -> NTSTATUS ; fn NtAlpcConnectPortEx (PortHandle : PHANDLE , ConnectionPortObjectAttributes : POBJECT_ATTRIBUTES , ClientPortObjectAttributes : POBJECT_ATTRIBUTES , PortAttributes : PALPC_PORT_ATTRIBUTES , Flags : ULONG , ServerSecurityRequirements : PSECURITY_DESCRIPTOR , ConnectionMessage : PPORT_MESSAGE , BufferLength : PSIZE_T , OutMessageAttributes : PALPC_MESSAGE_ATTRIBUTES , InMessageAttributes : PALPC_MESSAGE_ATTRIBUTES , Timeout : PLARGE_INTEGER ,) -> NTSTATUS ; fn NtAlpcAcceptConnectPort (PortHandle : PHANDLE , ConnectionPortHandle : HANDLE , Flags : ULONG , ObjectAttributes : POBJECT_ATTRIBUTES , PortAttributes : PALPC_PORT_ATTRIBUTES , PortContext : PVOID , ConnectionRequest : PPORT_MESSAGE , ConnectionMessageAttributes : PALPC_MESSAGE_ATTRIBUTES , AcceptConnection : BOOLEAN ,) -> NTSTATUS ; fn NtAlpcSendWaitReceivePort (PortHandle : HANDLE , Flags : ULONG , SendMessageA : PPORT_MESSAGE , SendMessageAttributes : PALPC_MESSAGE_ATTRIBUTES , ReceiveMessage : PPORT_MESSAGE , BufferLength : PSIZE_T , ReceiveMessageAttributes : PALPC_MESSAGE_ATTRIBUTES , Timeout : PLARGE_INTEGER ,) -> NTSTATUS ; } }
};
}
