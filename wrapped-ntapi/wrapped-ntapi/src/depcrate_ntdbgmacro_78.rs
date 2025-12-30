// Generated macro for macro_78 (macro)
macro_rules! Depcrate_ntdbgmacro_78 {
() => {
// Module: crate::ntdbg
// Provides: {"macro_78"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateDebugObject (DebugObjectHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , Flags : ULONG ,) -> NTSTATUS ; fn NtDebugActiveProcess (ProcessHandle : HANDLE , DebugObjectHandle : HANDLE ,) -> NTSTATUS ; fn NtDebugContinue (DebugObjectHandle : HANDLE , ClientId : PCLIENT_ID , ContinueStatus : NTSTATUS ,) -> NTSTATUS ; fn NtRemoveProcessDebug (ProcessHandle : HANDLE , DebugObjectHandle : HANDLE ,) -> NTSTATUS ; fn NtSetInformationDebugObject (DebugObjectHandle : HANDLE , DebugObjectInformationClass : DEBUGOBJECTINFOCLASS , DebugInformation : PVOID , DebugInformationLength : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; fn NtWaitForDebugEvent (DebugObjectHandle : HANDLE , Alertable : BOOLEAN , Timeout : PLARGE_INTEGER , WaitStateChange : PVOID ,) -> NTSTATUS ; fn DbgUiConnectToDbg () -> NTSTATUS ; fn DbgUiGetThreadDebugObject () -> HANDLE ; fn DbgUiSetThreadDebugObject (DebugObject : HANDLE ,) ; fn DbgUiWaitStateChange (StateChange : PDBGUI_WAIT_STATE_CHANGE , Timeout : PLARGE_INTEGER ,) -> NTSTATUS ; fn DbgUiContinue (AppClientId : PCLIENT_ID , ContinueStatus : NTSTATUS ,) -> NTSTATUS ; fn DbgUiStopDebugging (Process : HANDLE ,) -> NTSTATUS ; fn DbgUiDebugActiveProcess (Process : HANDLE ,) -> NTSTATUS ; fn DbgUiRemoteBreakin (Context : PVOID ,) ; fn DbgUiIssueRemoteBreakin (Process : HANDLE ,) -> NTSTATUS ; fn DbgUiConvertStateChangeStructure (StateChange : PDBGUI_WAIT_STATE_CHANGE , DebugEvent : LPDEBUG_EVENT ,) -> NTSTATUS ; } }
};
}
