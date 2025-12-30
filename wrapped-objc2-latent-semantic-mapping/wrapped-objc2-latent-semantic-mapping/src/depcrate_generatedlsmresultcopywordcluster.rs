// Generated macro for LSMResultCopyWordCluster (function)
macro_rules! Depcrate_generatedLSMResultCopyWordCluster {
() => {
// Module: crate::generated
// Provides: {"LSMResultCopyWordCluster"}
// Dependencies: {}
# [deprecated = "renamed to `LSMResult::word_cluster`"] # [inline] pub unsafe extern "C-unwind" fn LSMResultCopyWordCluster (result : & LSMResult , n : CFIndex ,) -> Option < CFRetained < CFArray > > { extern "C-unwind" { fn LSMResultCopyWordCluster (result : & LSMResult , n : CFIndex) -> Option < NonNull < CFArray > > ; } let ret = unsafe { LSMResultCopyWordCluster (result , n) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
