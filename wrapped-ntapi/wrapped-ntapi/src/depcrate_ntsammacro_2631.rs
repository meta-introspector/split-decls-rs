// Generated macro for macro_2631 (macro)
macro_rules! Depcrate_ntsammacro_2631 {
() => {
// Module: crate::ntsam
// Provides: {"macro_2631"}
// Dependencies: {}
EXTERN ! { extern "system" { fn SamEnumerateAliasesInDomain (DomainHandle : SAM_HANDLE , EnumerationContext : PSAM_ENUMERATE_HANDLE , Buffer : * mut PVOID , PreferedMaximumLength : ULONG , CountReturned : PULONG ,) -> NTSTATUS ; fn SamCreateAliasInDomain (DomainHandle : SAM_HANDLE , AccountName : PUNICODE_STRING , DesiredAccess : ACCESS_MASK , AliasHandle : PSAM_HANDLE , RelativeId : PULONG ,) -> NTSTATUS ; fn SamOpenAlias (DomainHandle : SAM_HANDLE , DesiredAccess : ACCESS_MASK , AliasId : ULONG , AliasHandle : PSAM_HANDLE ,) -> NTSTATUS ; fn SamDeleteAlias (AliasHandle : SAM_HANDLE ,) -> NTSTATUS ; fn SamQueryInformationAlias (AliasHandle : SAM_HANDLE , AliasInformationClass : ALIAS_INFORMATION_CLASS , Buffer : * mut PVOID ,) -> NTSTATUS ; fn SamSetInformationAlias (AliasHandle : SAM_HANDLE , AliasInformationClass : ALIAS_INFORMATION_CLASS , Buffer : PVOID ,) -> NTSTATUS ; fn SamAddMemberToAlias (AliasHandle : SAM_HANDLE , MemberId : PSID ,) -> NTSTATUS ; fn SamAddMultipleMembersToAlias (AliasHandle : SAM_HANDLE , MemberIds : * mut PSID , MemberCount : ULONG ,) -> NTSTATUS ; fn SamRemoveMemberFromAlias (AliasHandle : SAM_HANDLE , MemberId : PSID ,) -> NTSTATUS ; fn SamRemoveMultipleMembersFromAlias (AliasHandle : SAM_HANDLE , MemberIds : * mut PSID , MemberCount : ULONG ,) -> NTSTATUS ; fn SamGetMembersInAlias (AliasHandle : SAM_HANDLE , MemberIds : * mut * mut PSID , MemberCount : PULONG ,) -> NTSTATUS ; fn SamGetAliasMembership (DomainHandle : SAM_HANDLE , PassedCount : ULONG , Sids : * mut PSID , MembershipCount : PULONG , Aliases : * mut PULONG ,) -> NTSTATUS ; } }
};
}
