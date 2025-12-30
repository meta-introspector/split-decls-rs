// Generated macro for LSMResultCopyTokenCluster (function)
macro_rules! Depcrate_generatedLSMResultCopyTokenCluster {
() => {
// Module: crate::generated
// Provides: {"LSMResultCopyTokenCluster"}
// Dependencies: {}
# [deprecated = "renamed to `LSMResult::token_cluster`"] # [inline] pub unsafe extern "C-unwind" fn LSMResultCopyTokenCluster (result : & LSMResult , n : CFIndex ,) -> Option < CFRetained < CFArray > > { extern "C-unwind" { fn LSMResultCopyTokenCluster (result : & LSMResult , n : CFIndex) -> Option < NonNull < CFArray > > ; } let ret = unsafe { LSMResultCopyTokenCluster (result , n) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
