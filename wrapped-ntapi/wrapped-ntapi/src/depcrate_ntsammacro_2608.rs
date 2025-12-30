// Generated macro for macro_2608 (macro)
macro_rules! Depcrate_ntsammacro_2608 {
() => {
// Module: crate::ntsam
// Provides: {"macro_2608"}
// Dependencies: {}
EXTERN ! { extern "system" { fn SamEnumerateGroupsInDomain (DomainHandle : SAM_HANDLE , EnumerationContext : PSAM_ENUMERATE_HANDLE , Buffer : * mut PVOID , PreferedMaximumLength : ULONG , CountReturned : PULONG ,) -> NTSTATUS ; fn SamCreateGroupInDomain (DomainHandle : SAM_HANDLE , AccountName : PUNICODE_STRING , DesiredAccess : ACCESS_MASK , GroupHandle : PSAM_HANDLE , RelativeId : PULONG ,) -> NTSTATUS ; fn SamOpenGroup (DomainHandle : SAM_HANDLE , DesiredAccess : ACCESS_MASK , GroupId : ULONG , GroupHandle : PSAM_HANDLE ,) -> NTSTATUS ; fn SamDeleteGroup (GroupHandle : SAM_HANDLE ,) -> NTSTATUS ; fn SamQueryInformationGroup (GroupHandle : SAM_HANDLE , GroupInformationClass : GROUP_INFORMATION_CLASS , Buffer : * mut PVOID ,) -> NTSTATUS ; fn SamSetInformationGroup (GroupHandle : SAM_HANDLE , GroupInformationClass : GROUP_INFORMATION_CLASS , Buffer : PVOID ,) -> NTSTATUS ; fn SamAddMemberToGroup (GroupHandle : SAM_HANDLE , MemberId : ULONG , Attributes : ULONG ,) -> NTSTATUS ; fn SamRemoveMemberFromGroup (GroupHandle : SAM_HANDLE , MemberId : ULONG ,) -> NTSTATUS ; fn SamGetMembersInGroup (GroupHandle : SAM_HANDLE , MemberIds : * mut PULONG , Attributes : * mut PULONG , MemberCount : PULONG ,) -> NTSTATUS ; fn SamSetMemberAttributesOfGroup (GroupHandle : SAM_HANDLE , MemberId : ULONG , Attributes : ULONG ,) -> NTSTATUS ; } }
};
}
