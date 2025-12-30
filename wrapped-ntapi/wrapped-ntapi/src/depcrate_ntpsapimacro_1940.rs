// Generated macro for macro_1940 (macro)
macro_rules! Depcrate_ntpsapimacro_1940 {
() => {
// Module: crate::ntpsapi
// Provides: {"macro_1940"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateUserProcess (ProcessHandle : PHANDLE , ThreadHandle : PHANDLE , ProcessDesiredAccess : ACCESS_MASK , ThreadDesiredAccess : ACCESS_MASK , ProcessObjectAttributes : POBJECT_ATTRIBUTES , ThreadObjectAttributes : POBJECT_ATTRIBUTES , ProcessFlags : ULONG , ThreadFlags : ULONG , ProcessParameters : PVOID , CreateInfo : PPS_CREATE_INFO , AttributeList : PPS_ATTRIBUTE_LIST ,) -> NTSTATUS ; } }
};
}
