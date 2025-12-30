// Generated macro for RTL_CONTEXT_OFFSET (macro)
macro_rules! Depcrate_ntrtlRTL_CONTEXT_OFFSET {
() => {
// Module: crate::ntrtl
// Provides: {"RTL_CONTEXT_OFFSET"}
// Dependencies: {}
# [macro_export] macro_rules ! RTL_CONTEXT_OFFSET { ($ Context : expr , $ Chunk : ident) => { RTL_CONTEXT_EX_OFFSET ! (($ Context as * const $ crate :: winapi :: um :: winnt :: CONTEXT) . offset (1) as * const $ crate :: ntrtl :: CONTEXT_EX , $ Chunk) } ; }
};
}
