// Generated macro for macro_1947 (macro)
macro_rules! Depcrate_ntpsapimacro_1947 {
() => {
// Module: crate::ntpsapi
// Provides: {"macro_1947"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateThreadEx (ThreadHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , ProcessHandle : HANDLE , StartRoutine : PVOID , Argument : PVOID , CreateFlags : ULONG , ZeroBits : SIZE_T , StackSize : SIZE_T , MaximumStackSize : SIZE_T , AttributeList : PPS_ATTRIBUTE_LIST ,) -> NTSTATUS ; } }
};
}
