// Generated macro for macro_2587 (macro)
macro_rules! Depcrate_ntsammacro_2587 {
() => {
// Module: crate::ntsam
// Provides: {"macro_2587"}
// Dependencies: {}
EXTERN ! { extern "system" { fn SamLookupDomainInSamServer (ServerHandle : SAM_HANDLE , Name : PUNICODE_STRING , DomainId : * mut PSID ,) -> NTSTATUS ; fn SamEnumerateDomainsInSamServer (ServerHandle : SAM_HANDLE , EnumerationContext : PSAM_ENUMERATE_HANDLE , Buffer : * mut PVOID , PreferedMaximumLength : ULONG , CountReturned : PULONG ,) -> NTSTATUS ; fn SamOpenDomain (ServerHandle : SAM_HANDLE , DesiredAccess : ACCESS_MASK , DomainId : PSID , DomainHandle : PSAM_HANDLE ,) -> NTSTATUS ; fn SamQueryInformationDomain (DomainHandle : SAM_HANDLE , DomainInformationClass : DOMAIN_INFORMATION_CLASS , Buffer : * mut PVOID ,) -> NTSTATUS ; fn SamSetInformationDomain (DomainHandle : SAM_HANDLE , DomainInformationClass : DOMAIN_INFORMATION_CLASS , DomainInformation : PVOID ,) -> NTSTATUS ; fn SamLookupNamesInDomain (DomainHandle : SAM_HANDLE , Count : ULONG , Names : PUNICODE_STRING , RelativeIds : * mut PULONG , Use : * mut PSID_NAME_USE ,) -> NTSTATUS ; fn SamLookupIdsInDomain (DomainHandle : SAM_HANDLE , Count : ULONG , RelativeIds : PULONG , Names : * mut PUNICODE_STRING , Use : * mut PSID_NAME_USE ,) -> NTSTATUS ; fn SamRemoveMemberFromForeignDomain (DomainHandle : SAM_HANDLE , MemberId : PSID ,) -> NTSTATUS ; fn SamQueryLocalizableAccountsInDomain (Domain : SAM_HANDLE , Flags : ULONG , LanguageId : ULONG , Class : DOMAIN_LOCALIZABLE_ACCOUNTS_INFORMATION , Buffer : * mut PVOID ,) -> NTSTATUS ; } }
};
}
