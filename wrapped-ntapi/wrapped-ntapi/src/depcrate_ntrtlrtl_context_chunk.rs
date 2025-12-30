// Generated macro for RTL_CONTEXT_CHUNK (macro)
macro_rules! Depcrate_ntrtlRTL_CONTEXT_CHUNK {
() => {
// Module: crate::ntrtl
// Provides: {"RTL_CONTEXT_CHUNK"}
// Dependencies: {}
# [macro_export] macro_rules ! RTL_CONTEXT_CHUNK { ($ Context : expr , $ Chunk : ident) => { RTL_CONTEXT_EX_CHUNK ! (($ Context as * const $ crate :: winapi :: um :: winnt :: CONTEXT) . offset (1) as * const $ crate :: ntrtl :: CONTEXT_EX , ($ Context as * const $ crate :: winapi :: um :: winnt :: CONTEXT) . offset (1) as * const $ crate :: ntrtl :: CONTEXT_EX , $ Chunk) } ; }
};
}
