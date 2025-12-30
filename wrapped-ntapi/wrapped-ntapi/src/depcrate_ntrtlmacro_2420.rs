// Generated macro for macro_2420 (macro)
macro_rules! Depcrate_ntrtlmacro_2420 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2420"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlWalkFrameChain (Callers : * mut PVOID , Count : ULONG , Flags : ULONG ,) -> ULONG ; fn RtlGetCallersAddress (CallersAddress : * mut PVOID , CallersCaller : * mut PVOID ,) ; fn RtlGetEnabledExtendedFeatures (FeatureMask : ULONG64 ,) -> ULONG64 ; fn RtlGetEnabledExtendedAndSupervisorFeatures (FeatureMask : ULONG64 ,) -> ULONG64 ; fn RtlLocateSupervisorFeature (XStateHeader : PXSAVE_AREA_HEADER , FeatureId : ULONG , Length : PULONG ,) -> PVOID ; } }
};
}
