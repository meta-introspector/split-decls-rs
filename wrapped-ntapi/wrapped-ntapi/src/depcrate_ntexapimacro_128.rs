// Generated macro for macro_128 (macro)
macro_rules! Depcrate_ntexapimacro_128 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_128"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateMutant (MutantHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , InitialOwner : BOOLEAN ,) -> NTSTATUS ; fn NtOpenMutant (MutantHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES ,) -> NTSTATUS ; fn NtReleaseMutant (MutantHandle : HANDLE , PreviousCount : PLONG ,) -> NTSTATUS ; fn NtQueryMutant (MutantHandle : HANDLE , MutantInformationClass : MUTANT_INFORMATION_CLASS , MutantInformation : PVOID , MutantInformationLength : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; } }
};
}
