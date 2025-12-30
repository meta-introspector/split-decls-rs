// Generated macro for macro_1488 (macro)
macro_rules! Depcrate_ntobapimacro_1488 {
() => {
// Module: crate::ntobapi
// Provides: {"macro_1488"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtQueryDirectoryObject (DirectoryHandle : HANDLE , Buffer : PVOID , Length : ULONG , ReturnSingleEntry : BOOLEAN , RestartScan : BOOLEAN , Context : PULONG , ReturnLength : PULONG ,) -> NTSTATUS ; fn NtCreatePrivateNamespace (NamespaceHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , BoundaryDescriptor : PVOID ,) -> NTSTATUS ; fn NtOpenPrivateNamespace (NamespaceHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , BoundaryDescriptor : PVOID ,) -> NTSTATUS ; fn NtDeletePrivateNamespace (NamespaceHandle : HANDLE ,) -> NTSTATUS ; fn NtCreateSymbolicLinkObject (LinkHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , LinkTarget : PUNICODE_STRING ,) -> NTSTATUS ; fn NtOpenSymbolicLinkObject (LinkHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES ,) -> NTSTATUS ; fn NtQuerySymbolicLinkObject (LinkHandle : HANDLE , LinkTarget : PUNICODE_STRING , ReturnedLength : PULONG ,) -> NTSTATUS ; } }
};
}
