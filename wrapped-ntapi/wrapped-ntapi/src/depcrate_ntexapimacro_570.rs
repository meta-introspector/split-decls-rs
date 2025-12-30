// Generated macro for macro_570 (macro)
macro_rules! Depcrate_ntexapimacro_570 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_570"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtQueryDefaultLocale (UserProfile : BOOLEAN , DefaultLocaleId : PLCID ,) -> NTSTATUS ; fn NtSetDefaultLocale (UserProfile : BOOLEAN , DefaultLocaleId : LCID ,) -> NTSTATUS ; fn NtQueryInstallUILanguage (InstallUILanguageId : * mut LANGID ,) -> NTSTATUS ; fn NtFlushInstallUILanguage (InstallUILanguage : LANGID , SetComittedFlag : ULONG ,) -> NTSTATUS ; fn NtQueryDefaultUILanguage (DefaultUILanguageId : * mut LANGID ,) -> NTSTATUS ; fn NtSetDefaultUILanguage (DefaultUILanguageId : LANGID ,) -> NTSTATUS ; fn NtIsUILanguageComitted () -> NTSTATUS ; fn NtInitializeNlsFiles (BaseAddress : * mut PVOID , DefaultLocaleId : PLCID , DefaultCasingTableSize : PLARGE_INTEGER ,) -> NTSTATUS ; fn NtGetNlsSectionPtr (SectionType : ULONG , SectionData : ULONG , ContextData : PVOID , SectionPointer : * mut PVOID , SectionSize : PULONG ,) -> NTSTATUS ; fn NtMapCMFModule (What : ULONG , Index : ULONG , CacheIndexOut : PULONG , CacheFlagsOut : PULONG , ViewSizeOut : PULONG , BaseAddress : * mut PVOID ,) -> NTSTATUS ; fn NtGetMUIRegistryInfo (Flags : ULONG , DataSize : PULONG , Data : PVOID ,) -> NTSTATUS ; fn NtAddAtom (AtomName : PWSTR , Length : ULONG , Atom : PRTL_ATOM ,) -> NTSTATUS ; } }
};
}
