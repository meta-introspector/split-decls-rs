// Generated macro for LSMResultCopyToken (function)
macro_rules! Depcrate_generatedLSMResultCopyToken {
() => {
// Module: crate::generated
// Provides: {"LSMResultCopyToken"}
// Dependencies: {}
# [deprecated = "renamed to `LSMResult::token`"] # [inline] pub unsafe extern "C-unwind" fn LSMResultCopyToken (result : & LSMResult , n : CFIndex ,) -> Option < CFRetained < CFData > > { extern "C-unwind" { fn LSMResultCopyToken (result : & LSMResult , n : CFIndex) -> Option < NonNull < CFData > > ; } let ret = unsafe { LSMResultCopyToken (result , n) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
