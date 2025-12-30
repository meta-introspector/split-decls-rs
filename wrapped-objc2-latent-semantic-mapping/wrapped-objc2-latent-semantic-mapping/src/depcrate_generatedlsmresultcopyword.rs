// Generated macro for LSMResultCopyWord (function)
macro_rules! Depcrate_generatedLSMResultCopyWord {
() => {
// Module: crate::generated
// Provides: {"LSMResultCopyWord"}
// Dependencies: {}
# [deprecated = "renamed to `LSMResult::word`"] # [inline] pub unsafe extern "C-unwind" fn LSMResultCopyWord (result : & LSMResult , n : CFIndex ,) -> Option < CFRetained < CFString > > { extern "C-unwind" { fn LSMResultCopyWord (result : & LSMResult , n : CFIndex) -> Option < NonNull < CFString > > ; } let ret = unsafe { LSMResultCopyWord (result , n) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
