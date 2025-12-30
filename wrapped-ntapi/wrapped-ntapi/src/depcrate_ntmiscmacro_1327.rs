// Generated macro for macro_1327 (macro)
macro_rules! Depcrate_ntmiscmacro_1327 {
() => {
// Module: crate::ntmisc
// Provides: {"macro_1327"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtVdmControl (Service : VDMSERVICECLASS , ServiceData : PVOID ,) -> NTSTATUS ; fn NtTraceEvent (TraceHandle : HANDLE , Flags : ULONG , FieldSize : ULONG , Fields : PVOID ,) -> NTSTATUS ; fn NtTraceControl (FunctionCode : ULONG , InBuffer : PVOID , InBufferLen : ULONG , OutBuffer : PVOID , OutBufferLen : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; } }
};
}
