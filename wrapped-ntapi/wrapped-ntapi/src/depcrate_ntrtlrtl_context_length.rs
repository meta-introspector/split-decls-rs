// Generated macro for RTL_CONTEXT_LENGTH (macro)
macro_rules! Depcrate_ntrtlRTL_CONTEXT_LENGTH {
() => {
// Module: crate::ntrtl
// Provides: {"RTL_CONTEXT_LENGTH"}
// Dependencies: {}
# [macro_export] macro_rules ! RTL_CONTEXT_LENGTH { ($ Context : expr , $ Chunk : ident) => { RTL_CONTEXT_EX_LENGTH ! (($ Context as * const $ crate :: winapi :: um :: winnt :: CONTEXT) . offset (1) as * const $ crate :: ntrtl :: CONTEXT_EX , $ Chunk) } ; }
};
}
