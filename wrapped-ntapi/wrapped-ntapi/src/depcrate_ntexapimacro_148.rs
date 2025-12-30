// Generated macro for macro_148 (macro)
macro_rules! Depcrate_ntexapimacro_148 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_148"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateProfile (ProfileHandle : PHANDLE , Process : HANDLE , ProfileBase : PVOID , ProfileSize : SIZE_T , BucketSize : ULONG , Buffer : PULONG , BufferSize : ULONG , ProfileSource : KPROFILE_SOURCE , Affinity : KAFFINITY ,) -> NTSTATUS ; fn NtCreateProfileEx (ProfileHandle : PHANDLE , Process : HANDLE , ProfileBase : PVOID , ProfileSize : SIZE_T , BucketSize : ULONG , Buffer : PULONG , BufferSize : ULONG , ProfileSource : KPROFILE_SOURCE , GroupCount : USHORT , GroupAffinity : PGROUP_AFFINITY ,) -> NTSTATUS ; fn NtStartProfile (ProfileHandle : HANDLE ,) -> NTSTATUS ; fn NtStopProfile (ProfileHandle : HANDLE ,) -> NTSTATUS ; fn NtQueryIntervalProfile (ProfileSource : KPROFILE_SOURCE , Interval : PULONG ,) -> NTSTATUS ; fn NtSetIntervalProfile (Interval : ULONG , Source : KPROFILE_SOURCE ,) -> NTSTATUS ; } }
};
}
