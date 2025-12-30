// Generated macro for macro_1639 (macro)
macro_rules! Depcrate_ntpoapimacro_1639 {
() => {
// Module: crate::ntpoapi
// Provides: {"macro_1639"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtPowerInformation (InformationLevel : POWER_INFORMATION_LEVEL , InputBuffer : PVOID , InputBufferLength : ULONG , OutputBuffer : PVOID , OutputBufferLength : ULONG ,) -> NTSTATUS ; fn NtSetThreadExecutionState (NewFlags : EXECUTION_STATE , PreviousFlags : PEXECUTION_STATE ,) -> NTSTATUS ; fn NtRequestWakeupLatency (latency : LATENCY_TIME ,) -> NTSTATUS ; fn NtInitiatePowerAction (SystemAction : POWER_ACTION , LightestSystemState : SYSTEM_POWER_STATE , Flags : ULONG , Asynchronous : BOOLEAN ,) -> NTSTATUS ; fn NtSetSystemPowerState (SystemAction : POWER_ACTION , LightestSystemState : SYSTEM_POWER_STATE , Flags : ULONG ,) -> NTSTATUS ; fn NtGetDevicePowerState (Device : HANDLE , State : PDEVICE_POWER_STATE ,) -> NTSTATUS ; fn NtIsSystemResumeAutomatic () -> BOOLEAN ; } }
};
}
