// Generated macro for macro_2216 (macro)
macro_rules! Depcrate_ntrtlmacro_2216 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2216"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlInitializeContext (Process : HANDLE , Context : PCONTEXT , Parameter : PVOID , InitialPc : PVOID , InitialSp : PVOID ,) ; fn RtlInitializeExtendedContext (Context : PCONTEXT , ContextFlags : ULONG , ContextEx : * mut PCONTEXT_EX ,) -> ULONG ; fn RtlCopyExtendedContext (Destination : PCONTEXT_EX , ContextFlags : ULONG , Source : PCONTEXT_EX ,) -> ULONG ; fn RtlGetExtendedContextLength (ContextFlags : ULONG , ContextLength : PULONG ,) -> ULONG ; fn RtlGetExtendedFeaturesMask (ContextEx : PCONTEXT_EX ,) -> ULONG64 ; fn RtlLocateExtendedFeature (ContextEx : PCONTEXT_EX , FeatureId : ULONG , Length : PULONG ,) -> PVOID ; fn RtlLocateLegacyContext (ContextEx : PCONTEXT_EX , Length : PULONG ,) -> PCONTEXT ; fn RtlSetExtendedFeaturesMask (ContextEx : PCONTEXT_EX , FeatureMask : ULONG64 ,) ; } }
};
}
