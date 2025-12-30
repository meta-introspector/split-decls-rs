// Generated macro for macro_1611 (macro)
macro_rules! Depcrate_ntpnpapimacro_1611 {
() => {
// Module: crate::ntpnpapi
// Provides: {"macro_1611"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtGetPlugPlayEvent (EventHandle : HANDLE , Context : PVOID , EventBlock : PPLUGPLAY_EVENT_BLOCK , EventBufferSize : ULONG ,) -> NTSTATUS ; fn NtPlugPlayControl (PnPControlClass : PLUGPLAY_CONTROL_CLASS , PnPControlData : PVOID , PnPControlDataLength : ULONG ,) -> NTSTATUS ; fn NtSerializeBoot () -> NTSTATUS ; fn NtEnableLastKnownGood () -> NTSTATUS ; fn NtDisableLastKnownGood () -> NTSTATUS ; fn NtReplacePartitionUnit (TargetInstancePath : PUNICODE_STRING , SpareInstancePath : PUNICODE_STRING , Flags : ULONG ,) -> NTSTATUS ; } }
};
}
